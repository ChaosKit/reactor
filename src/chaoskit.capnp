@0xd3927750ee8b980a;

using Cxx = import "/capnp/c++.capnp";
$Cxx.namespace("chaoskit");

struct Message {
  type @0 :MessageType;
  body :union {
    none @1 :Void;
    flame @2 :Flame;
  }
}

enum MessageType {
  start @0;
  stop @1;
}

struct Flame {
  transforms @0 :List(Transform);
  finalTransform @1 :Transform;
  resetTransformation @3 :AffineTransformation;

  ttl @2 :Int32;
}

struct Transform {
  variations @0 :List(Variation);
  weight @1 :Float64 = 1.0;

  pre @3 :AffineTransformation;
  post @4 :AffineTransformation;

  coloringMethod :union {
    distance @5 :Void;
    singleColor @2 :Float64 = 0.5;
  }
}

struct Variation {
  name @0 :Text;
  params @1 :List(Float64);
  weight @2 :Extent = (x = 1.0, y = 1.0);
}

struct AffineTransformation {
  translation @0 :Extent = (x = 0.0, y = 0.0);
  scale @1 :Extent = (x = 1.0, y = 1.0);
  rotation @2 :Float64 = 0.0;
}

struct Extent {
  x @0 :Float64;
  y @1 :Float64;
}
