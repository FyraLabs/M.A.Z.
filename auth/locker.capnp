@0x8bbf2af9c2bf08ae;

struct Key {
    id @0 :Text;
    # alg @? :AlgType;
    issuer @1 :Text; # Optional, "" => None
    accountName @2 :Text;
    # digits @? :UInt8;
    secret @3 :Data; # Vec<u8>
    

    # enum AlgType {
    #     sha1 @0;
    # }
}

struct Locker {
    name @0 :Text;
    keys @1 :List(Key);
}
