#define PY_SSIZE_T_CLEAN
#include <Python.h>

static PyObject *cvarint_encode(PyObject *self, PyObject *args) {
  unsigned long long n;
  char out[10];
  if (!PyArg_ParseTuple(args, "K", &n)) {
    return NULL;
  }
  int i = 0;
  while (n > 0) {
    char part = n & 0x7f;
    n >>= 7;
    if (n > 0) {
      part |= 0x80;
    }
    out[i++] = part;
  }
  return PyBytes_FromStringAndSize(out, i);
}

static PyObject *cvarint_decode(PyObject *self, PyObject *args) {
  const char *varn;
  Py_ssize_t length;

  if (!PyArg_ParseTuple(args, "y#", &varn, &length)) {
    return NULL;
  }

  unsigned long long res = 0;
  for (size_t i = length; i > 0; i--) {
    char b = varn[i - 1];
    res <<= 7;
    res |= b & 0x7f;
  }

  return PyLong_FromUnsignedLongLong(res);
}

static PyMethodDef CVarintMethods[] = {
    {"encode", cvarint_encode, METH_VARARGS, "Encode an integer as varint."},
    {"decode", cvarint_decode, METH_VARARGS,
     "Decode varint bytes to an integer."},
    {NULL, NULL, 0, NULL}};

static struct PyModuleDef cvarintmodule = {
    PyModuleDef_HEAD_INIT, "cvarint",
    "A C implementation of protobuf varint encoding", -1, CVarintMethods};

PyMODINIT_FUNC PyInit_cvarint(void) { return PyModule_Create(&cvarintmodule); }