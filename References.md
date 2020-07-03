
A reference represents a borrow of some owned value. You can get one by using the & or &mut operators on a value, or by using a ref or ref mut pattern.

For those familiar with pointers, a reference is just a pointer that is assumed to be aligned, not null, and pointing to memory containing a valid value of T - for example, &bool can only point to an allocation containing the integer values 1 (true) or 0 (false), but creating a &bool that points to an allocation containing the value 3 causes undefined behaviour. In fact, Option<&T> has the same memory representation as a nullable but aligned pointer, and can be passed across FFI boundaries as such.

In most cases, references can be used much like the original value. Field access, method calling, and indexing work the same (save for mutability rules, of course). In addition, the comparison operators transparently defer to the referent's implementation, allowing references to be compared the same as owned values.

References have a lifetime attached to them, which represents the scope for which the borrow is valid. A lifetime is said to "outlive" another one if its representative scope is as long or longer than the other. The 'static lifetime is the longest lifetime, which represents the total life of the program. For example, string literals have a 'static lifetime because the text data is embedded into the binary of the program, rather than in an allocation that needs to be dynamically managed.

