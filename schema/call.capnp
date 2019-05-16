@0xf4908ff0f100947d;

struct Presence {
  role @0 :Role;
  name @1 :Text;

  enum Role {
    driver @0;
    vehicle @1;
  }
}
