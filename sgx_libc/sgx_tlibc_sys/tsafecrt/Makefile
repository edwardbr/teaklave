#
# Copyright (C) 2011-2021 Intel Corporation. All rights reserved.
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions
# are met:
#
#   * Redistributions of source code must retain the above copyright
#     notice, this list of conditions and the following disclaimer.
#   * Redistributions in binary form must reproduce the above copyright
#     notice, this list of conditions and the following disclaimer in
#     the documentation and/or other materials provided with the
#     distribution.
#   * Neither the name of Intel Corporation nor the names of its
#     contributors may be used to endorse or promote products derived
#     from this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
# "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
# LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
# A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
# OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
# SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
# LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
# DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
# THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
# (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
# OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
#
#

include ../../../buildenv.mk

OUT_DIR ?= ./out

SGX_COMMON_FLAGS := -O2 -D_FORTIFY_SOURCE=2 -UDEBUG -DNDEBUG
CFLAGS   += $(SGX_COMMON_FLAGS) $(ENCLAVE_CFLAGS) -D _LIBSAFECRT_SGX_CONFIG
CFLAGS   += -std=c99
CXXFLAGS += $(SGX_COMMON_FLAGS) $(ENCLAVE_CXXFLAGS)

CPPFLAGS += -I.                          \
            -I$(COMMON_DIR)/inc          \
            -I$(COMMON_DIR)/inc/tlibc    \
            -I./pal/inc                  \

ifneq ($(ARCH), x86)
CFLAGS += -D _WIN64
endif

LIBC_SRCS := $(wildcard pal/src/safecrt/*.c)
LIBCPP_SRCS := $(wildcard pal/src/*.cpp)

LIB_OBJS := $(LIBC_SRCS:.c=.o)
LIB_OBJS += $(LIBCPP_SRCS:.cpp=.o)
LIB_OBJS := $(sort $(LIB_OBJS))
LIB_OBJS := $(addprefix $(OUT_DIR)/, $(LIB_OBJS))

LIB_NAME := $(OUT_DIR)/libtsafecrt.a
BUILD_DIR := build_dir

.NOTPARALLEL:

.PHONY: all
all: $(BUILD_DIR) $(LIB_NAME)

$(LIB_NAME): $(LIB_OBJS)
	$(AR) rcs $@ $^

$(OUT_DIR)/%.o: %.c
	$(CC)  $(CFLAGS)   $(CPPFLAGS) -c $< -o $@

$(OUT_DIR)/%.o: %.cpp
	$(CXX) $(CXXFLAGS) $(CPPFLAGS) -c $< -o $@
   
$(BUILD_DIR):
	$(MKDIR) $(OUT_DIR)
	$(MKDIR) $(OUT_DIR)/pal
	$(MKDIR) $(OUT_DIR)/pal/src
	$(MKDIR) $(OUT_DIR)/pal/src/safecrt

.PHONY: clean
clean:
	@$(RM) $(LIB_NAME) $(LIB_OBJS)
