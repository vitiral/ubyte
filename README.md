# ubyte: rust library for serialization/deserialization on microcontrollers

ubyte (micro-byte) is a rust library implementing serde traits for
serialization and deseriaization on microcontrollers. Its goals are:
- Easy to use. Serde lets you use `#[derive(Serialize, Deserialize)]`
  and be done.
- Known maximum value. It should be possible (although not currently
  determinable) to determine the maximum amount of space a datatype can take up.
  Several features of normal serialization libraries are intentionally NOT
  supported such as vectors and maps as they could be of an unknown size.
- Full featured for what makes sense. Supported types include all
  floats/integers/etc, structs, nested structs, tuples and nums up to 255
  variants (1 byte)
- Zero allocated memory (data is serialized to/from buffers only)

ubyte is very similar to the library bincode except that it intentionally has
fewer features and targets a much more "micro" design space. If you are not
communicating to/from microcontrollers I recommend you use bincode instead.

The basic design is:
- structs/tuples/arrays are serialized in the order of their fields/indexes
  into a big endian tightly packed byte array. IF YOU CHANGE THE ORDER OF FIELDS
  FOR EITHER STRUCTS OR ENUMS YOU WILL BREAK COMPATIBILITY WITH OLDER VERSIONS
  OF YOUR OWN LIBRARY. YOU HAVE BEEN WARNED.
- enum variants are stored as a single byte (u8) representing the index of the
  variant. Enum variants with values are the u8 index followed by whatever the
  value is.
