rust-cql-protoco
========================
Experimental CQL protocol implementation. 



So some explanation:

When building an outgoing packet, the flow is:
Tell Stream what what kind of frame to send (Stream.startup().
If it's a frame that sends data, or otherwise takes arguments, pass them in.
That method will then

1. do all work necessary to figure out the body len()
2. build a Frame with a Vec<> presized to be exactly the size of the Frame (body + 9 bytes)
3. Finish building the fixed size (9 byte) header struct
4. transmute that struct into Vec<u8> and push_all the bytes into the Vec<u8>
5. If the body was not fully built, push_all any existing bytes into the FrameVec<u8> 
6. keep pushing bytes into the vec until you are done.

If that was all done right, the Vec<u8> was preallocated to the exact right size.

We then just send the frame.bytes across the socket

Building in inbound packet is even more fun:

1. always assume the reader can start reading at the beginning of a frame boundary
2. read 9 bytes into a [u8]
3. transmute that [u8] into a byte-for-byte identical Header struct
4. read the Length field of the header
5. allocate a Vec<u8> of exactly the size that the header tells us this frame will be (Length + 9)
6. push_all the header bytes (if you still have them as bytes, otherwise transmute your struct back to bytes
7. read Length bytes from the socket 
