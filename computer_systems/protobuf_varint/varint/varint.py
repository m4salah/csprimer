import struct


def main():
    for n in range(1 << 30):
        print("Testing number", n)
        assert decode(encode(n)) == n

    with open("1.uint64", "rb") as f:
        n = struct.unpack('>Q', f.read())[0]
        assert encode(n) == b'\x01'

    with open("150.uint64", "rb") as f:
        n = struct.unpack('>Q', f.read())[0]
        assert encode(n) == b'\x96\x01'

    with open("maxint.uint64", "rb") as f:
        n = struct.unpack('>Q', f.read())[0]

    print('OK!')


def rchunks(str, n: int):
    str = str[::-1]
    return [str[i:i+n][::-1] for i in range(0, len(str), n)][::-1]


def fillbits(arr: [str]):
    newArr = []
    for s in arr:
        newS = ''.join(['0' for i in range(8 - len(s))]) + s
        newArr.append(newS)

    # change msp to 1 but the first byte
    for i, s in enumerate(newArr):
        if i != 0:
            newLast = list(newArr[i])
            newLast[0] = '1'
            newArr[i] = ''.join(newLast)
    return ''.join(newArr[::-1])


def unfillbits(arr: [str]):
    # drop the msp in every element
    return "".join(list(map(lambda x: x[1:], arr))[::-1])


def encode(n: int) -> bytes:
    """
    This is the encoding fucntion for variable width integer (varints)
    for more information
    https://protobuf.dev/programming-guides/encoding/#varints
    while n > 0:
        - take the lowest order 7 bits
        - and add to it the correct msp: 1 for all but the final 7 bits
        - push it to some sequence of bytes
        - reduce n by 7 bits
    return the sequence we constructing
    """
    out = []
    while n > 0:
        # Take the lowest 7 bits, TODO: we can mask it later.
        part = n & 0x7F  # This is equevilant to 0b01111111
        n >>= 7
        if n > 0:
            # turn the msp to 1
            part |= (1 << 7)  # This is equevilant to 0b10000000 (0x80)
        out.append(part)

    return bytes(out)


def decode(varn: bytes):
    """
    This is the decode function of decoding a sequence of bytes (varints)
    and decode it to normal integer

    https://protobuf.dev/programming-guides/encoding/#varints
    for v in reversed varn:
        - shift the accumlator to the left 7 bits
        - drop the msp
        - add the v after droping the msp to the accumlator
    return the accumlator.
    """

    n = 0
    for v in reversed(varn):
        n <<= 7
        n |= (v & 0x7F)

    return n


if __name__ == "__main__":
    main()
