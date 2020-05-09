# Basic Rust
> There are no strict constraints on which part of the object store the object can access, because there are no restrictions on the way the object references are passed around.

This has repercussions when preventing representation exposure for aggregate objects.

A better solution can be to restrict the visibility of different types of objects that are created. This is done by saying that all objects have a context associated with them. All paths from the root of the system must pass though the objects' owner.