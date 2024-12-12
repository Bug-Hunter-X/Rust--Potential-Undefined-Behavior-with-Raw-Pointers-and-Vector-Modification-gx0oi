# Rust: Potential Undefined Behavior with Raw Pointers and Vector Modification

This repository demonstrates a potential undefined behavior in Rust when dealing with raw pointers and vector modification. Modifying the vector's data through a raw pointer after operations that might reallocate the vector's memory can lead to unexpected issues, including memory corruption.

## Bug Description
The original code attempts to modify the contents of a vector using a raw pointer. This approach is unsafe, because any vector operations after acquiring the raw pointer might cause the vector to reallocate the memory, thus invalidating the pointer.  Access through the invalid pointer is undefined behavior.

## Solution
The solution avoids unsafe code and instead uses safe methods to modify the vector.  This ensures memory safety and prevents potential crashes or unexpected behavior.