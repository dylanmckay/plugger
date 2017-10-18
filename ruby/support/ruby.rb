# A mixin that makes an object seem like it contains Rust methods.
module Dispatcher
  def methods
    super.map { |method| ((/(.*)_internal/ =~ method.to_s) ? $1 : method).to_sym }
  end

  def method_missing(name, *original_args)
    return super unless methods.include?(name)
    method_pointer = self.pointer_to_function(name)

    if method_pointer
      # we have C functions plugged in, with the '_internal' suffix.
      # here, we call the internal shim and give it the pointer
      # to the Rust function.
      internal_method = "#{name}_internal".to_sym

      validate_arguments!(method(internal_method), original_args)

      arguments = [method_pointer] + original_args

      send(internal_method, *arguments)
    else
      super
    end
  end

  private

  # Raise an ArgumentError if the given arguments don't match the callee.
  #
  # We check this in Rubyland because if we call the internal method
  # and the arguments are wrong, the ArgumentError message will have
  # the hidden function pointer included.
  def validate_arguments!(method, original_args)
    # we don't want to include the hidden method pointer we pass
    non_self_arg_count = method.arity - 1

    if non_self_arg_count != original_args.size
      raise ArgumentError, "wrong number of arguments (given #{original_args.size}, expected #{non_self_arg_count})"
    end
  end
end

class PluggerObject
  include Dispatcher # dispatch methods
  extend Dispatcher  # dispatch singleton methods

  attr_reader :object_pointer

  def initialize(object_pointer)
    @object_pointer = object_pointer
  end

  protected

  # Gets a pointer to a function.
  # Alias of the class method.
  def pointer_to_function(name)
    self.class.pointer_to_function(name)
  end

  # Gets a pointer to a function.
  def self.pointer_to_function(name)
    pointer_const_name = name.to_s.upcase
    self.const_get(pointer_const_name)
  end
end
