[Exposed=*]
interface ReadableStreamDefaultReader {
  // constructor(ReadableStream stream);

  // Promise<ReadableStreamReadResult> read();
  // undefined releaseLock();
};
ReadableStreamDefaultReader includes ReadableStreamGenericReader;

dictionary ReadableStreamReadResult {
  any value;
  boolean done;
};