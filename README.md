> Rust library for Java to decrypt classes on definition, utilising a hook in the JVM.

# JVM ClassLoader hook

## How it works
This library enhances the JVM by creating a hook when loaded in the `JVM_DefineClassWithSource` export in the `jvm` library.<br>
This feature gives us the ability to read and modify the bytes of a class before it's constructed, providing greater control and customization options.

## Note
Please note that this is a basic implementation of class decryption in Java.<br>
While this method may provide a first line of defense against immediate decompiling, it's important to understand that more advanced tools and agents may still be able to read and modify the loaded classes.<br>
Keep this in mind as you use this feature in your projects.

## Compatibility
### ✅ Java
The export `JVM_DefineClassWithSource` was only added in JDK1.5, therefore all Java versions starting from `Java 6` are supported (there is no JNI version for `Java 5`).<br>
Thus far only `Java 8`, `Java 20` and `Java 21` were explicitly tested.

### ✅ Operating system
The build targets all major architectures
- Linux
- Windows
- MacOS

# How to build
1. [Install rust](https://www.rust-lang.org/tools/install)
2. Install nightly by running `rustup install nightly`
3. Enable nightly by running `rustup override set nightly`
4. Run `cargo build --release`

# How to use
## Integration
Load the library from the defined path like:
```java
System.loadLibrary("classloader");
```

## Encryption
### Define your custom magic value.
The value is used by the classloader hook to determine if the given payload uses the encryption method defined in the `ClassLoader::is_custom_payload` method.<br>
*Note: Changes to this value also require corresponding changes in the constant in `class_loader.rs`.*
```java
private static final byte[] CUSTOM_MAGIC_VALUE = BigInteger.valueOf(0xDEADC0DE).toByteArray();
```

### Encrypt the bytes of your class
For simplicity, every class is XOR encrypted using the same static key "`my static key`". Add `ENCRYPTION_KEY` as environment variable (or secret in GitHub) to take control over this value.<br>
Changing the entire encryption part is highly recommended if you want to use a custom build. Modify `ClassLoader::decrypt_payload` to reverse your encryption method.
```java
final byte[] keyBytes = "my static key".getBytes();
for (int i = 0; i < classBytes.length; i++) {
    classBytes[i] ^= keyBytes[i % keyBytes.length];
}
```

### Override javas magic value with your custom one
This allows the library to detect a custom class, to forward it to the decryption procedure.
```java
System.arraycopy(CUSTOM_MAGIC_VALUE, 0, classBytes, 0, CUSTOM_MAGIC_VALUE.length);
```
