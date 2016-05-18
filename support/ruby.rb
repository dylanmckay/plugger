class PluggerObject
  attr_reader :object_pointer

  def initialize(object_pointer)
    @object_pointer = object_pointer
  end

  def method_missing(name, *args)
    val = Kernel.const_get("#{self.class.name}::#{name.upcase}")
    super unless val

    internal_method = "#{name}_internal".to_sym
    send(internal_method, val)
  end
end
