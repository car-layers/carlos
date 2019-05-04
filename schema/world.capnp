@0x86a77c0a9ad9101e;

struct View {
  objects @0 :List(Object);

  struct Object {
    points @0 :List(Point);
	
    struct Point {
	  distance @1 :UInt16;
	  angle @0 :UInt16;
	}
  }
}
