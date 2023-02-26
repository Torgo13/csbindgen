#[allow(unused)]
use ::std::os::raw::*;
use super::lz4;

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_versionNumber(
) ->  c_int
{
    unsafe {
        return lz4::LZ4_versionNumber(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_versionString(
) -> *const c_char
{
    unsafe {
        return lz4::LZ4_versionString(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_default(
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    dstCapacity:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_default(
            src,
            dst,
            srcSize,
            dstCapacity,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_safe(
    src: *const c_char,
    dst: *mut c_char,
    compressedSize:  c_int,
    dstCapacity:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_safe(
            src,
            dst,
            compressedSize,
            dstCapacity,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressBound(
    inputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressBound(
            inputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_fast(
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    dstCapacity:  c_int,
    acceleration:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_fast(
            src,
            dst,
            srcSize,
            dstCapacity,
            acceleration,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_sizeofState(
) ->  c_int
{
    unsafe {
        return lz4::LZ4_sizeofState(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_fast_extState(
    state: *mut c_void,
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    dstCapacity:  c_int,
    acceleration:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_fast_extState(
            state,
            src,
            dst,
            srcSize,
            dstCapacity,
            acceleration,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_destSize(
    src: *const c_char,
    dst: *mut c_char,
    srcSizePtr: *mut c_int,
    targetDstSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_destSize(
            src,
            dst,
            srcSizePtr,
            targetDstSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_safe_partial(
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    targetOutputSize:  c_int,
    dstCapacity:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_safe_partial(
            src,
            dst,
            srcSize,
            targetOutputSize,
            dstCapacity,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_createStream(
) -> *mut lz4::LZ4_stream_t
{
    unsafe {
        return lz4::LZ4_createStream(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_freeStream(
    streamPtr: *mut lz4::LZ4_stream_t,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_freeStream(
            streamPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_resetStream_fast(
    streamPtr: *mut lz4::LZ4_stream_t,
)
{
    unsafe {
        return lz4::LZ4_resetStream_fast(
            streamPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_loadDict(
    streamPtr: *mut lz4::LZ4_stream_t,
    dictionary: *const c_char,
    dictSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_loadDict(
            streamPtr,
            dictionary,
            dictSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_fast_continue(
    streamPtr: *mut lz4::LZ4_stream_t,
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    dstCapacity:  c_int,
    acceleration:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_fast_continue(
            streamPtr,
            src,
            dst,
            srcSize,
            dstCapacity,
            acceleration,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_saveDict(
    streamPtr: *mut lz4::LZ4_stream_t,
    safeBuffer: *mut c_char,
    maxDictSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_saveDict(
            streamPtr,
            safeBuffer,
            maxDictSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_createStreamDecode(
) -> *mut lz4::LZ4_streamDecode_t
{
    unsafe {
        return lz4::LZ4_createStreamDecode(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_freeStreamDecode(
    LZ4_stream: *mut lz4::LZ4_streamDecode_t,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_freeStreamDecode(
            LZ4_stream,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_setStreamDecode(
    LZ4_streamDecode: *mut lz4::LZ4_streamDecode_t,
    dictionary: *const c_char,
    dictSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_setStreamDecode(
            LZ4_streamDecode,
            dictionary,
            dictSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decoderRingBufferSize(
    maxBlockSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decoderRingBufferSize(
            maxBlockSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_safe_continue(
    LZ4_streamDecode: *mut lz4::LZ4_streamDecode_t,
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    dstCapacity:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_safe_continue(
            LZ4_streamDecode,
            src,
            dst,
            srcSize,
            dstCapacity,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_safe_usingDict(
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    dstCapacity:  c_int,
    dictStart: *const c_char,
    dictSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_safe_usingDict(
            src,
            dst,
            srcSize,
            dstCapacity,
            dictStart,
            dictSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_safe_partial_usingDict(
    src: *const c_char,
    dst: *mut c_char,
    compressedSize:  c_int,
    targetOutputSize:  c_int,
    maxOutputSize:  c_int,
    dictStart: *const c_char,
    dictSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_safe_partial_usingDict(
            src,
            dst,
            compressedSize,
            targetOutputSize,
            maxOutputSize,
            dictStart,
            dictSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen___va_start(
    arg1: *mut *mut c_char,
)
{
    unsafe {
        return lz4::__va_start(
            arg1,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen___security_init_cookie(
)
{
    unsafe {
        return lz4::__security_init_cookie(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen___security_check_cookie(
    _StackCookie:  usize,
)
{
    unsafe {
        return lz4::__security_check_cookie(
            _StackCookie,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_initStream(
    buffer: *mut c_void,
    size:  usize,
) -> *mut lz4::LZ4_stream_t
{
    unsafe {
        return lz4::LZ4_initStream(
            buffer,
            size,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress(
    src: *const c_char,
    dest: *mut c_char,
    srcSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress(
            src,
            dest,
            srcSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_limitedOutput(
    src: *const c_char,
    dest: *mut c_char,
    srcSize:  c_int,
    maxOutputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_limitedOutput(
            src,
            dest,
            srcSize,
            maxOutputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_withState(
    state: *mut c_void,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_withState(
            state,
            source,
            dest,
            inputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_limitedOutput_withState(
    state: *mut c_void,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    maxOutputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_limitedOutput_withState(
            state,
            source,
            dest,
            inputSize,
            maxOutputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_continue(
    LZ4_streamPtr: *mut lz4::LZ4_stream_t,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_continue(
            LZ4_streamPtr,
            source,
            dest,
            inputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_limitedOutput_continue(
    LZ4_streamPtr: *mut lz4::LZ4_stream_t,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    maxOutputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_limitedOutput_continue(
            LZ4_streamPtr,
            source,
            dest,
            inputSize,
            maxOutputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_uncompress(
    source: *const c_char,
    dest: *mut c_char,
    outputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_uncompress(
            source,
            dest,
            outputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_uncompress_unknownOutputSize(
    source: *const c_char,
    dest: *mut c_char,
    isize_:  c_int,
    maxOutputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_uncompress_unknownOutputSize(
            source,
            dest,
            isize_,
            maxOutputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_create(
    inputBuffer: *mut c_char,
) -> *mut c_void
{
    unsafe {
        return lz4::LZ4_create(
            inputBuffer,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_sizeofStreamState(
) ->  c_int
{
    unsafe {
        return lz4::LZ4_sizeofStreamState(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_resetStreamState(
    state: *mut c_void,
    inputBuffer: *mut c_char,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_resetStreamState(
            state,
            inputBuffer,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_slideInputBuffer(
    state: *mut c_void,
) -> *mut c_char
{
    unsafe {
        return lz4::LZ4_slideInputBuffer(
            state,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_safe_withPrefix64k(
    src: *const c_char,
    dst: *mut c_char,
    compressedSize:  c_int,
    maxDstSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_safe_withPrefix64k(
            src,
            dst,
            compressedSize,
            maxDstSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_fast_withPrefix64k(
    src: *const c_char,
    dst: *mut c_char,
    originalSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_fast_withPrefix64k(
            src,
            dst,
            originalSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_fast(
    src: *const c_char,
    dst: *mut c_char,
    originalSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_fast(
            src,
            dst,
            originalSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_fast_continue(
    LZ4_streamDecode: *mut lz4::LZ4_streamDecode_t,
    src: *const c_char,
    dst: *mut c_char,
    originalSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_fast_continue(
            LZ4_streamDecode,
            src,
            dst,
            originalSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_decompress_fast_usingDict(
    src: *const c_char,
    dst: *mut c_char,
    originalSize:  c_int,
    dictStart: *const c_char,
    dictSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_decompress_fast_usingDict(
            src,
            dst,
            originalSize,
            dictStart,
            dictSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_resetStream(
    streamPtr: *mut lz4::LZ4_stream_t,
)
{
    unsafe {
        return lz4::LZ4_resetStream(
            streamPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_HC(
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    dstCapacity:  c_int,
    compressionLevel:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_HC(
            src,
            dst,
            srcSize,
            dstCapacity,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_sizeofStateHC(
) ->  c_int
{
    unsafe {
        return lz4::LZ4_sizeofStateHC(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_HC_extStateHC(
    stateHC: *mut c_void,
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    maxDstSize:  c_int,
    compressionLevel:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_HC_extStateHC(
            stateHC,
            src,
            dst,
            srcSize,
            maxDstSize,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_HC_destSize(
    stateHC: *mut c_void,
    src: *const c_char,
    dst: *mut c_char,
    srcSizePtr: *mut c_int,
    targetDstSize:  c_int,
    compressionLevel:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_HC_destSize(
            stateHC,
            src,
            dst,
            srcSizePtr,
            targetDstSize,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_createStreamHC(
) -> *mut lz4::LZ4_streamHC_t
{
    unsafe {
        return lz4::LZ4_createStreamHC(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_freeStreamHC(
    streamHCPtr: *mut lz4::LZ4_streamHC_t,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_freeStreamHC(
            streamHCPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_resetStreamHC_fast(
    streamHCPtr: *mut lz4::LZ4_streamHC_t,
    compressionLevel:  c_int,
)
{
    unsafe {
        return lz4::LZ4_resetStreamHC_fast(
            streamHCPtr,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_loadDictHC(
    streamHCPtr: *mut lz4::LZ4_streamHC_t,
    dictionary: *const c_char,
    dictSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_loadDictHC(
            streamHCPtr,
            dictionary,
            dictSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_HC_continue(
    streamHCPtr: *mut lz4::LZ4_streamHC_t,
    src: *const c_char,
    dst: *mut c_char,
    srcSize:  c_int,
    maxDstSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_HC_continue(
            streamHCPtr,
            src,
            dst,
            srcSize,
            maxDstSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compress_HC_continue_destSize(
    LZ4_streamHCPtr: *mut lz4::LZ4_streamHC_t,
    src: *const c_char,
    dst: *mut c_char,
    srcSizePtr: *mut c_int,
    targetDstSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compress_HC_continue_destSize(
            LZ4_streamHCPtr,
            src,
            dst,
            srcSizePtr,
            targetDstSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_saveDictHC(
    streamHCPtr: *mut lz4::LZ4_streamHC_t,
    safeBuffer: *mut c_char,
    maxDictSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_saveDictHC(
            streamHCPtr,
            safeBuffer,
            maxDictSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_initStreamHC(
    buffer: *mut c_void,
    size:  usize,
) -> *mut lz4::LZ4_streamHC_t
{
    unsafe {
        return lz4::LZ4_initStreamHC(
            buffer,
            size,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC(
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC(
            source,
            dest,
            inputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC_limitedOutput(
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    maxOutputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC_limitedOutput(
            source,
            dest,
            inputSize,
            maxOutputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC2(
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    compressionLevel:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC2(
            source,
            dest,
            inputSize,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC2_limitedOutput(
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    maxOutputSize:  c_int,
    compressionLevel:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC2_limitedOutput(
            source,
            dest,
            inputSize,
            maxOutputSize,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC_withStateHC(
    state: *mut c_void,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC_withStateHC(
            state,
            source,
            dest,
            inputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC_limitedOutput_withStateHC(
    state: *mut c_void,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    maxOutputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC_limitedOutput_withStateHC(
            state,
            source,
            dest,
            inputSize,
            maxOutputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC2_withStateHC(
    state: *mut c_void,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    compressionLevel:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC2_withStateHC(
            state,
            source,
            dest,
            inputSize,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC2_limitedOutput_withStateHC(
    state: *mut c_void,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    maxOutputSize:  c_int,
    compressionLevel:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC2_limitedOutput_withStateHC(
            state,
            source,
            dest,
            inputSize,
            maxOutputSize,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC_continue(
    LZ4_streamHCPtr: *mut lz4::LZ4_streamHC_t,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC_continue(
            LZ4_streamHCPtr,
            source,
            dest,
            inputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC_limitedOutput_continue(
    LZ4_streamHCPtr: *mut lz4::LZ4_streamHC_t,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    maxOutputSize:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC_limitedOutput_continue(
            LZ4_streamHCPtr,
            source,
            dest,
            inputSize,
            maxOutputSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_createHC(
    inputBuffer: *const c_char,
) -> *mut c_void
{
    unsafe {
        return lz4::LZ4_createHC(
            inputBuffer,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_freeHC(
    LZ4HC_Data: *mut c_void,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_freeHC(
            LZ4HC_Data,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_slideInputBufferHC(
    LZ4HC_Data: *mut c_void,
) -> *mut c_char
{
    unsafe {
        return lz4::LZ4_slideInputBufferHC(
            LZ4HC_Data,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC2_continue(
    LZ4HC_Data: *mut c_void,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    compressionLevel:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC2_continue(
            LZ4HC_Data,
            source,
            dest,
            inputSize,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_compressHC2_limitedOutput_continue(
    LZ4HC_Data: *mut c_void,
    source: *const c_char,
    dest: *mut c_char,
    inputSize:  c_int,
    maxOutputSize:  c_int,
    compressionLevel:  c_int,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_compressHC2_limitedOutput_continue(
            LZ4HC_Data,
            source,
            dest,
            inputSize,
            maxOutputSize,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_sizeofStreamStateHC(
) ->  c_int
{
    unsafe {
        return lz4::LZ4_sizeofStreamStateHC(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_resetStreamStateHC(
    state: *mut c_void,
    inputBuffer: *mut c_char,
) ->  c_int
{
    unsafe {
        return lz4::LZ4_resetStreamStateHC(
            state,
            inputBuffer,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4_resetStreamHC(
    streamHCPtr: *mut lz4::LZ4_streamHC_t,
    compressionLevel:  c_int,
)
{
    unsafe {
        return lz4::LZ4_resetStreamHC(
            streamHCPtr,
            compressionLevel,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_isError(
    code:  lz4::LZ4F_errorCode_t,
) ->  c_uint
{
    unsafe {
        return lz4::LZ4F_isError(
            code,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_getErrorName(
    code:  lz4::LZ4F_errorCode_t,
) -> *const c_char
{
    unsafe {
        return lz4::LZ4F_getErrorName(
            code,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_compressionLevel_max(
) ->  c_int
{
    unsafe {
        return lz4::LZ4F_compressionLevel_max(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_compressFrameBound(
    srcSize:  usize,
    preferencesPtr: *const lz4::LZ4F_preferences_t,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_compressFrameBound(
            srcSize,
            preferencesPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_compressFrame(
    dstBuffer: *mut c_void,
    dstCapacity:  usize,
    srcBuffer: *const c_void,
    srcSize:  usize,
    preferencesPtr: *const lz4::LZ4F_preferences_t,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_compressFrame(
            dstBuffer,
            dstCapacity,
            srcBuffer,
            srcSize,
            preferencesPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_getVersion(
) ->  c_uint
{
    unsafe {
        return lz4::LZ4F_getVersion(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_createCompressionContext(
    cctxPtr: *mut *mut lz4::LZ4F_cctx,
    version:  c_uint,
) ->  lz4::LZ4F_errorCode_t
{
    unsafe {
        return lz4::LZ4F_createCompressionContext(
            cctxPtr,
            version,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_freeCompressionContext(
    cctx: *mut lz4::LZ4F_cctx,
) ->  lz4::LZ4F_errorCode_t
{
    unsafe {
        return lz4::LZ4F_freeCompressionContext(
            cctx,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_compressBegin(
    cctx: *mut lz4::LZ4F_cctx,
    dstBuffer: *mut c_void,
    dstCapacity:  usize,
    prefsPtr: *const lz4::LZ4F_preferences_t,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_compressBegin(
            cctx,
            dstBuffer,
            dstCapacity,
            prefsPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_compressBound(
    srcSize:  usize,
    prefsPtr: *const lz4::LZ4F_preferences_t,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_compressBound(
            srcSize,
            prefsPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_compressUpdate(
    cctx: *mut lz4::LZ4F_cctx,
    dstBuffer: *mut c_void,
    dstCapacity:  usize,
    srcBuffer: *const c_void,
    srcSize:  usize,
    cOptPtr: *const lz4::LZ4F_compressOptions_t,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_compressUpdate(
            cctx,
            dstBuffer,
            dstCapacity,
            srcBuffer,
            srcSize,
            cOptPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_flush(
    cctx: *mut lz4::LZ4F_cctx,
    dstBuffer: *mut c_void,
    dstCapacity:  usize,
    cOptPtr: *const lz4::LZ4F_compressOptions_t,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_flush(
            cctx,
            dstBuffer,
            dstCapacity,
            cOptPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_compressEnd(
    cctx: *mut lz4::LZ4F_cctx,
    dstBuffer: *mut c_void,
    dstCapacity:  usize,
    cOptPtr: *const lz4::LZ4F_compressOptions_t,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_compressEnd(
            cctx,
            dstBuffer,
            dstCapacity,
            cOptPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_createDecompressionContext(
    dctxPtr: *mut *mut lz4::LZ4F_dctx,
    version:  c_uint,
) ->  lz4::LZ4F_errorCode_t
{
    unsafe {
        return lz4::LZ4F_createDecompressionContext(
            dctxPtr,
            version,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_freeDecompressionContext(
    dctx: *mut lz4::LZ4F_dctx,
) ->  lz4::LZ4F_errorCode_t
{
    unsafe {
        return lz4::LZ4F_freeDecompressionContext(
            dctx,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_headerSize(
    src: *const c_void,
    srcSize:  usize,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_headerSize(
            src,
            srcSize,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_getFrameInfo(
    dctx: *mut lz4::LZ4F_dctx,
    frameInfoPtr: *mut lz4::LZ4F_frameInfo_t,
    srcBuffer: *const c_void,
    srcSizePtr: *mut usize,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_getFrameInfo(
            dctx,
            frameInfoPtr,
            srcBuffer,
            srcSizePtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_decompress(
    dctx: *mut lz4::LZ4F_dctx,
    dstBuffer: *mut c_void,
    dstSizePtr: *mut usize,
    srcBuffer: *const c_void,
    srcSizePtr: *mut usize,
    dOptPtr: *const lz4::LZ4F_decompressOptions_t,
) ->  usize
{
    unsafe {
        return lz4::LZ4F_decompress(
            dctx,
            dstBuffer,
            dstSizePtr,
            srcBuffer,
            srcSizePtr,
            dOptPtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_LZ4F_resetDecompressionContext(
    dctx: *mut lz4::LZ4F_dctx,
)
{
    unsafe {
        return lz4::LZ4F_resetDecompressionContext(
            dctx,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH_versionNumber(
) ->  c_uint
{
    unsafe {
        return lz4::XXH_versionNumber(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH32(
    input: *const c_void,
    length:  usize,
    seed:  c_uint,
) ->  lz4::XXH32_hash_t
{
    unsafe {
        return lz4::XXH32(
            input,
            length,
            seed,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH32_createState(
) -> *mut lz4::XXH32_state_t
{
    unsafe {
        return lz4::XXH32_createState(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH32_freeState(
    statePtr: *mut lz4::XXH32_state_t,
) ->  lz4::XXH_errorcode
{
    unsafe {
        return lz4::XXH32_freeState(
            statePtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH32_copyState(
    dst_state: *mut lz4::XXH32_state_t,
    src_state: *const lz4::XXH32_state_t,
)
{
    unsafe {
        return lz4::XXH32_copyState(
            dst_state,
            src_state,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH32_reset(
    statePtr: *mut lz4::XXH32_state_t,
    seed:  c_uint,
) ->  lz4::XXH_errorcode
{
    unsafe {
        return lz4::XXH32_reset(
            statePtr,
            seed,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH32_update(
    statePtr: *mut lz4::XXH32_state_t,
    input: *const c_void,
    length:  usize,
) ->  lz4::XXH_errorcode
{
    unsafe {
        return lz4::XXH32_update(
            statePtr,
            input,
            length,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH32_digest(
    statePtr: *const lz4::XXH32_state_t,
) ->  lz4::XXH32_hash_t
{
    unsafe {
        return lz4::XXH32_digest(
            statePtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH32_canonicalFromHash(
    dst: *mut lz4::XXH32_canonical_t,
    hash:  lz4::XXH32_hash_t,
)
{
    unsafe {
        return lz4::XXH32_canonicalFromHash(
            dst,
            hash,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH32_hashFromCanonical(
    src: *const lz4::XXH32_canonical_t,
) ->  lz4::XXH32_hash_t
{
    unsafe {
        return lz4::XXH32_hashFromCanonical(
            src,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH64(
    input: *const c_void,
    length:  usize,
    seed:  c_ulonglong,
) ->  lz4::XXH64_hash_t
{
    unsafe {
        return lz4::XXH64(
            input,
            length,
            seed,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH64_createState(
) -> *mut lz4::XXH64_state_t
{
    unsafe {
        return lz4::XXH64_createState(
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH64_freeState(
    statePtr: *mut lz4::XXH64_state_t,
) ->  lz4::XXH_errorcode
{
    unsafe {
        return lz4::XXH64_freeState(
            statePtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH64_copyState(
    dst_state: *mut lz4::XXH64_state_t,
    src_state: *const lz4::XXH64_state_t,
)
{
    unsafe {
        return lz4::XXH64_copyState(
            dst_state,
            src_state,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH64_reset(
    statePtr: *mut lz4::XXH64_state_t,
    seed:  c_ulonglong,
) ->  lz4::XXH_errorcode
{
    unsafe {
        return lz4::XXH64_reset(
            statePtr,
            seed,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH64_update(
    statePtr: *mut lz4::XXH64_state_t,
    input: *const c_void,
    length:  usize,
) ->  lz4::XXH_errorcode
{
    unsafe {
        return lz4::XXH64_update(
            statePtr,
            input,
            length,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH64_digest(
    statePtr: *const lz4::XXH64_state_t,
) ->  lz4::XXH64_hash_t
{
    unsafe {
        return lz4::XXH64_digest(
            statePtr,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH64_canonicalFromHash(
    dst: *mut lz4::XXH64_canonical_t,
    hash:  lz4::XXH64_hash_t,
)
{
    unsafe {
        return lz4::XXH64_canonicalFromHash(
            dst,
            hash,
        )
    }
}

#[no_mangle]
pub extern "C" fn csbindgen_XXH64_hashFromCanonical(
    src: *const lz4::XXH64_canonical_t,
) ->  lz4::XXH64_hash_t
{
    unsafe {
        return lz4::XXH64_hashFromCanonical(
            src,
        )
    }
}

