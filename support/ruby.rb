class PluggerObject
  attr_reader :object_pointer

  def initialize(object_pointer)
    @object_pointer = object_pointer
  end

  def method_missing(name, *original_args)
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

  # TODO: do this for singleton_methods too
  def methods
    super.map do |method|
      if /(.*)_internal/ =~ method.to_s
        $1
      else
        method
      end.to_sym
    end
  end

  def self.pointer_to_function(name)
    pointer_const_name = name.to_s.upcase
    self.const_get(pointer_const_name)
  end
end
