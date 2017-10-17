module PluggerInternal
  def self.normalize_method_names(names)
    names.map { |method| ((/(.*)_internal/ =~ method.to_s) ? $1 : method).to_sym }
  end
end

class PluggerObject
  attr_reader :object_pointer

  def initialize(object_pointer)
    @object_pointer = object_pointer
  end

  def methods
    PluggerInternal.normalize_method_names(super)
  end

  def self.methods
    PluggerInternal.normalize_method_names(super)
  end

  def method_missing(name, *original_args)
    return super unless methods.include?(name)
    method_pointer = self.class.pointer_to_function(name)

    if method_pointer
      # we have C functions plugged in, with the '_internal' suffix.
      # here, we call the internal shim and give it the pointer
      # to the Rust function.
      internal_method = "#{name}_internal".to_sym

      arguments = [method_pointer] + original_args

      send(internal_method, *arguments)
    else
      super
    end
  end

  def self.method_missing(name, *original_args)
    return super unless methods.include?(name)
    method_pointer = self.pointer_to_function(name)

    if method_pointer
      # we have C functions plugged in, with the '_internal' suffix.
      # here, we call the internal shim and give it the pointer
      # to the Rust function.
      internal_method = "#{name}_internal".to_sym

      arguments = [method_pointer] + original_args

      send(internal_method, *arguments)
    else
      super
    end
  end

  def self.pointer_to_function(name)
    pointer_const_name = name.to_s.upcase
    self.const_get(pointer_const_name)
  end
end
