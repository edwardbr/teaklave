/*
 * Copyright (C) 2011-2021 Intel Corporation. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 *   * Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 *   * Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in
 *     the documentation and/or other materials provided with the
 *     distribution.
 *   * Neither the name of Intel Corporation nor the names of its
 *     contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#include <sgx_spinlock.h>

static inline void _mm_pause(void) __attribute__((always_inline));
static inline int _InterlockedExchange(int volatile * dst, int val) __attribute__((always_inline));

static inline void _mm_pause(void)  /* definition requires -ffreestanding */
{
    __asm __volatile(
        "pause"
    );
}

static inline int _InterlockedExchange(int volatile * dst, int val)
{
    int res;

    __asm __volatile(
        "lock xchg %2, %1;"
        "mov %2, %0"
        : "=m" (res)
        : "m" (*dst),
        "r" (val) 
        : "memory"
    );

    return (res);
   
}

#define MIN_BACKOFF 2
#define MAX_BACKOFF 1024
uint32_t sgx_spin_lock(sgx_spinlock_t *lock)
{
     while(_InterlockedExchange((volatile int *)lock, 1) != 0) {
        int b = MIN_BACKOFF;
        do
        {    /* tell cpu we are spinning */
            for (int i=0; i < b; i++) {
                _mm_pause();
            }
            b <<= 1;
            if (b > MAX_BACKOFF) {
                b = MAX_BACKOFF;
            }
        } while (*lock);
    }
    return (0);
}

uint32_t sgx_spin_unlock(sgx_spinlock_t *lock)
{
    *lock = 0;

    return (0);
}
