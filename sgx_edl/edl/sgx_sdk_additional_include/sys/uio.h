//
// Copyright © 2005-2020 Rich Felker, et al.
// Licensed under the MIT license.
//

/* Copyright © 2005-2020 Rich Felker, et al.

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. */

#ifndef _SYS_UIO_H_
#define _SYS_UIO_H_

#include <sys/cdefs.h>
#include <sys/types.h>

struct iovec {
    void *iov_base;
    size_t iov_len;
};

__BEGIN_DECLS

// ocall
ssize_t readv(int, const struct iovec *, int);
ssize_t writev(int, const struct iovec *, int);

ssize_t preadv64(int, const struct iovec *, int, uint64_t);
ssize_t pwritev64(int, const struct iovec *, int, uint64_t);

__END_DECLS

#endif /* _SYS_UIO_H_ */
