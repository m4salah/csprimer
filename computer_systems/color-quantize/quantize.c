unsigned char quantize(unsigned char red, unsigned char green,
                       unsigned char blue) {
  return (red & 0b11100000) + ((green & 0b11100000) >> 3) +
         ((blue & 0b11000000) >> 6);
}
