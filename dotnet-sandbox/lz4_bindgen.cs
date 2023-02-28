// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
using System;
using System.Runtime.InteropServices;

namespace CsBindgen
{
    public static unsafe partial class LibLz4
    {
#if UNITY_IOS && !UNITY_EDITOR
        const string __DllName = "__Internal";
#else
        const string __DllName = "csbindgen_tests";
#endif
        

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_versionNumber", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_versionNumber();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_versionString", CallingConvention = CallingConvention.Cdecl)]
        public static extern byte* LZ4_versionString();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_default", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_default(byte* src, byte* dst, int srcSize, int dstCapacity);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_safe", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_safe(byte* src, byte* dst, int compressedSize, int dstCapacity);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressBound", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressBound(int inputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_fast", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_fast(byte* src, byte* dst, int srcSize, int dstCapacity, int acceleration);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_sizeofState", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_sizeofState();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_fast_extState", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_fast_extState(void* state, byte* src, byte* dst, int srcSize, int dstCapacity, int acceleration);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_destSize", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_destSize(byte* src, byte* dst, int* srcSizePtr, int targetDstSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_safe_partial", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_safe_partial(byte* src, byte* dst, int srcSize, int targetOutputSize, int dstCapacity);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_createStream", CallingConvention = CallingConvention.Cdecl)]
        public static extern LZ4_stream_u* LZ4_createStream();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_freeStream", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_freeStream(LZ4_stream_u* streamPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_resetStream_fast", CallingConvention = CallingConvention.Cdecl)]
        public static extern void LZ4_resetStream_fast(LZ4_stream_u* streamPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_loadDict", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_loadDict(LZ4_stream_u* streamPtr, byte* dictionary, int dictSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_fast_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_fast_continue(LZ4_stream_u* streamPtr, byte* src, byte* dst, int srcSize, int dstCapacity, int acceleration);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_saveDict", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_saveDict(LZ4_stream_u* streamPtr, byte* safeBuffer, int maxDictSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_createStreamDecode", CallingConvention = CallingConvention.Cdecl)]
        public static extern LZ4_streamDecode_u* LZ4_createStreamDecode();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_freeStreamDecode", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_freeStreamDecode(LZ4_streamDecode_u* LZ4_stream);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_setStreamDecode", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_setStreamDecode(LZ4_streamDecode_u* LZ4_streamDecode, byte* dictionary, int dictSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decoderRingBufferSize", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decoderRingBufferSize(int maxBlockSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_safe_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_safe_continue(LZ4_streamDecode_u* LZ4_streamDecode, byte* src, byte* dst, int srcSize, int dstCapacity);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_safe_usingDict", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_safe_usingDict(byte* src, byte* dst, int srcSize, int dstCapacity, byte* dictStart, int dictSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_safe_partial_usingDict", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_safe_partial_usingDict(byte* src, byte* dst, int compressedSize, int targetOutputSize, int maxOutputSize, byte* dictStart, int dictSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_initStream", CallingConvention = CallingConvention.Cdecl)]
        public static extern LZ4_stream_u* LZ4_initStream(void* buffer, UIntPtr size);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress(byte* src, byte* dest, int srcSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_limitedOutput", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_limitedOutput(byte* src, byte* dest, int srcSize, int maxOutputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_withState", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_withState(void* state, byte* source, byte* dest, int inputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_limitedOutput_withState", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_limitedOutput_withState(void* state, byte* source, byte* dest, int inputSize, int maxOutputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_continue(LZ4_stream_u* LZ4_streamPtr, byte* source, byte* dest, int inputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_limitedOutput_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_limitedOutput_continue(LZ4_stream_u* LZ4_streamPtr, byte* source, byte* dest, int inputSize, int maxOutputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_uncompress", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_uncompress(byte* source, byte* dest, int outputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_uncompress_unknownOutputSize", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_uncompress_unknownOutputSize(byte* source, byte* dest, int isize_, int maxOutputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_create", CallingConvention = CallingConvention.Cdecl)]
        public static extern void* LZ4_create(byte* inputBuffer);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_sizeofStreamState", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_sizeofStreamState();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_resetStreamState", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_resetStreamState(void* state, byte* inputBuffer);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_slideInputBuffer", CallingConvention = CallingConvention.Cdecl)]
        public static extern byte* LZ4_slideInputBuffer(void* state);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_safe_withPrefix64k", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_safe_withPrefix64k(byte* src, byte* dst, int compressedSize, int maxDstSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_fast_withPrefix64k", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_fast_withPrefix64k(byte* src, byte* dst, int originalSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_fast", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_fast(byte* src, byte* dst, int originalSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_fast_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_fast_continue(LZ4_streamDecode_u* LZ4_streamDecode, byte* src, byte* dst, int originalSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_decompress_fast_usingDict", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_decompress_fast_usingDict(byte* src, byte* dst, int originalSize, byte* dictStart, int dictSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_resetStream", CallingConvention = CallingConvention.Cdecl)]
        public static extern void LZ4_resetStream(LZ4_stream_u* streamPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_HC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_HC(byte* src, byte* dst, int srcSize, int dstCapacity, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_sizeofStateHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_sizeofStateHC();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_HC_extStateHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_HC_extStateHC(void* stateHC, byte* src, byte* dst, int srcSize, int maxDstSize, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_HC_destSize", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_HC_destSize(void* stateHC, byte* src, byte* dst, int* srcSizePtr, int targetDstSize, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_createStreamHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern LZ4_streamHC_u* LZ4_createStreamHC();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_freeStreamHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_freeStreamHC(LZ4_streamHC_u* streamHCPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_resetStreamHC_fast", CallingConvention = CallingConvention.Cdecl)]
        public static extern void LZ4_resetStreamHC_fast(LZ4_streamHC_u* streamHCPtr, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_loadDictHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_loadDictHC(LZ4_streamHC_u* streamHCPtr, byte* dictionary, int dictSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_HC_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_HC_continue(LZ4_streamHC_u* streamHCPtr, byte* src, byte* dst, int srcSize, int maxDstSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compress_HC_continue_destSize", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compress_HC_continue_destSize(LZ4_streamHC_u* LZ4_streamHCPtr, byte* src, byte* dst, int* srcSizePtr, int targetDstSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_saveDictHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_saveDictHC(LZ4_streamHC_u* streamHCPtr, byte* safeBuffer, int maxDictSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_initStreamHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern LZ4_streamHC_u* LZ4_initStreamHC(void* buffer, UIntPtr size);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC(byte* source, byte* dest, int inputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC_limitedOutput", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC_limitedOutput(byte* source, byte* dest, int inputSize, int maxOutputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC2", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC2(byte* source, byte* dest, int inputSize, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC2_limitedOutput", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC2_limitedOutput(byte* source, byte* dest, int inputSize, int maxOutputSize, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC_withStateHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC_withStateHC(void* state, byte* source, byte* dest, int inputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC_limitedOutput_withStateHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC_limitedOutput_withStateHC(void* state, byte* source, byte* dest, int inputSize, int maxOutputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC2_withStateHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC2_withStateHC(void* state, byte* source, byte* dest, int inputSize, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC2_limitedOutput_withStateHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC2_limitedOutput_withStateHC(void* state, byte* source, byte* dest, int inputSize, int maxOutputSize, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC_continue(LZ4_streamHC_u* LZ4_streamHCPtr, byte* source, byte* dest, int inputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC_limitedOutput_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC_limitedOutput_continue(LZ4_streamHC_u* LZ4_streamHCPtr, byte* source, byte* dest, int inputSize, int maxOutputSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_createHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern void* LZ4_createHC(byte* inputBuffer);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_freeHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_freeHC(void* LZ4HC_Data);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_slideInputBufferHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern byte* LZ4_slideInputBufferHC(void* LZ4HC_Data);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC2_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC2_continue(void* LZ4HC_Data, byte* source, byte* dest, int inputSize, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_compressHC2_limitedOutput_continue", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_compressHC2_limitedOutput_continue(void* LZ4HC_Data, byte* source, byte* dest, int inputSize, int maxOutputSize, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_sizeofStreamStateHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_sizeofStreamStateHC();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_resetStreamStateHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4_resetStreamStateHC(void* state, byte* inputBuffer);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4_resetStreamHC", CallingConvention = CallingConvention.Cdecl)]
        public static extern void LZ4_resetStreamHC(LZ4_streamHC_u* streamHCPtr, int compressionLevel);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_isError", CallingConvention = CallingConvention.Cdecl)]
        public static extern uint LZ4F_isError(UIntPtr code);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_getErrorName", CallingConvention = CallingConvention.Cdecl)]
        public static extern byte* LZ4F_getErrorName(UIntPtr code);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_compressionLevel_max", CallingConvention = CallingConvention.Cdecl)]
        public static extern int LZ4F_compressionLevel_max();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_compressFrameBound", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_compressFrameBound(UIntPtr srcSize, LZ4F_preferences_t* preferencesPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_compressFrame", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_compressFrame(void* dstBuffer, UIntPtr dstCapacity, void* srcBuffer, UIntPtr srcSize, LZ4F_preferences_t* preferencesPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_getVersion", CallingConvention = CallingConvention.Cdecl)]
        public static extern uint LZ4F_getVersion();

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_createCompressionContext", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_createCompressionContext(LZ4F_cctx_s** cctxPtr, uint version);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_freeCompressionContext", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_freeCompressionContext(LZ4F_cctx_s* cctx);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_compressBegin", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_compressBegin(LZ4F_cctx_s* cctx, void* dstBuffer, UIntPtr dstCapacity, LZ4F_preferences_t* prefsPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_compressBound", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_compressBound(UIntPtr srcSize, LZ4F_preferences_t* prefsPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_compressUpdate", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_compressUpdate(LZ4F_cctx_s* cctx, void* dstBuffer, UIntPtr dstCapacity, void* srcBuffer, UIntPtr srcSize, LZ4F_compressOptions_t* cOptPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_flush", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_flush(LZ4F_cctx_s* cctx, void* dstBuffer, UIntPtr dstCapacity, LZ4F_compressOptions_t* cOptPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_compressEnd", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_compressEnd(LZ4F_cctx_s* cctx, void* dstBuffer, UIntPtr dstCapacity, LZ4F_compressOptions_t* cOptPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_createDecompressionContext", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_createDecompressionContext(LZ4F_dctx_s** dctxPtr, uint version);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_freeDecompressionContext", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_freeDecompressionContext(LZ4F_dctx_s* dctx);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_headerSize", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_headerSize(void* src, UIntPtr srcSize);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_getFrameInfo", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_getFrameInfo(LZ4F_dctx_s* dctx, LZ4F_frameInfo_t* frameInfoPtr, void* srcBuffer, UIntPtr* srcSizePtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_decompress", CallingConvention = CallingConvention.Cdecl)]
        public static extern UIntPtr LZ4F_decompress(LZ4F_dctx_s* dctx, void* dstBuffer, UIntPtr* dstSizePtr, void* srcBuffer, UIntPtr* srcSizePtr, LZ4F_decompressOptions_t* dOptPtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_LZ4F_resetDecompressionContext", CallingConvention = CallingConvention.Cdecl)]
        public static extern void LZ4F_resetDecompressionContext(LZ4F_dctx_s* dctx);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH_versionNumber", CallingConvention = CallingConvention.Cdecl)]
        public static extern uint XXH_versionNumber();

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH32", CallingConvention = CallingConvention.Cdecl)]
        public static extern uint XXH32(void* input, UIntPtr length, uint seed);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH32_createState", CallingConvention = CallingConvention.Cdecl)]
        public static extern XXH32_state_s* XXH32_createState();

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH32_freeState", CallingConvention = CallingConvention.Cdecl)]
        public static extern int XXH32_freeState(XXH32_state_s* statePtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH32_copyState", CallingConvention = CallingConvention.Cdecl)]
        public static extern void XXH32_copyState(XXH32_state_s* dst_state, XXH32_state_s* src_state);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH32_reset", CallingConvention = CallingConvention.Cdecl)]
        public static extern int XXH32_reset(XXH32_state_s* statePtr, uint seed);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH32_update", CallingConvention = CallingConvention.Cdecl)]
        public static extern int XXH32_update(XXH32_state_s* statePtr, void* input, UIntPtr length);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH32_digest", CallingConvention = CallingConvention.Cdecl)]
        public static extern uint XXH32_digest(XXH32_state_s* statePtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH32_canonicalFromHash", CallingConvention = CallingConvention.Cdecl)]
        public static extern void XXH32_canonicalFromHash(XXH32_canonical_t* dst, uint hash);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH32_hashFromCanonical", CallingConvention = CallingConvention.Cdecl)]
        public static extern uint XXH32_hashFromCanonical(XXH32_canonical_t* src);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH64", CallingConvention = CallingConvention.Cdecl)]
        public static extern ulong XXH64(void* input, UIntPtr length, ulong seed);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH64_createState", CallingConvention = CallingConvention.Cdecl)]
        public static extern XXH64_state_s* XXH64_createState();

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH64_freeState", CallingConvention = CallingConvention.Cdecl)]
        public static extern int XXH64_freeState(XXH64_state_s* statePtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH64_copyState", CallingConvention = CallingConvention.Cdecl)]
        public static extern void XXH64_copyState(XXH64_state_s* dst_state, XXH64_state_s* src_state);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH64_reset", CallingConvention = CallingConvention.Cdecl)]
        public static extern int XXH64_reset(XXH64_state_s* statePtr, ulong seed);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH64_update", CallingConvention = CallingConvention.Cdecl)]
        public static extern int XXH64_update(XXH64_state_s* statePtr, void* input, UIntPtr length);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH64_digest", CallingConvention = CallingConvention.Cdecl)]
        public static extern ulong XXH64_digest(XXH64_state_s* statePtr);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH64_canonicalFromHash", CallingConvention = CallingConvention.Cdecl)]
        public static extern void XXH64_canonicalFromHash(XXH64_canonical_t* dst, ulong hash);

        [DllImport(__DllName, EntryPoint = "csbindgen_XXH64_hashFromCanonical", CallingConvention = CallingConvention.Cdecl)]
        public static extern ulong XXH64_hashFromCanonical(XXH64_canonical_t* src);


    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct LZ4_stream_t_internal
    {
        public fixed uint hashTable[4096];
        public byte* dictionary;
        public LZ4_stream_t_internal* dictCtx;
        public uint currentOffset;
        public uint tableType;
        public uint dictSize;
    }

    [StructLayout(LayoutKind.Explicit)]
    public unsafe struct LZ4_stream_u
    {
        [FieldOffset(0)]
        public fixed byte minStateSize[16416];
        [FieldOffset(0)]
        public LZ4_stream_t_internal internal_donotuse;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct LZ4_streamDecode_t_internal
    {
        public byte* externalDict;
        public byte* prefixEnd;
        public UIntPtr extDictSize;
        public UIntPtr prefixSize;
    }

    [StructLayout(LayoutKind.Explicit)]
    public unsafe struct LZ4_streamDecode_u
    {
        [FieldOffset(0)]
        public fixed byte minStateSize[32];
        [FieldOffset(0)]
        public LZ4_streamDecode_t_internal internal_donotuse;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct LZ4HC_CCtx_internal
    {
        public fixed uint hashTable[32768];
        public fixed ushort chainTable[65536];
        public byte* end;
        public byte* prefixStart;
        public byte* dictStart;
        public uint dictLimit;
        public uint lowLimit;
        public uint nextToUpdate;
        public short compressionLevel;
        public sbyte favorDecSpeed;
        public sbyte dirty;
        public LZ4HC_CCtx_internal* dictCtx;
    }

    [StructLayout(LayoutKind.Explicit)]
    public unsafe struct LZ4_streamHC_u
    {
        [FieldOffset(0)]
        public fixed byte minStateSize[262200];
        [FieldOffset(0)]
        public LZ4HC_CCtx_internal internal_donotuse;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct LZ4F_frameInfo_t
    {
        public int blockSizeID;
        public int blockMode;
        public int contentChecksumFlag;
        public int frameType;
        public ulong contentSize;
        public uint dictID;
        public int blockChecksumFlag;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct LZ4F_preferences_t
    {
        public LZ4F_frameInfo_t frameInfo;
        public int compressionLevel;
        public uint autoFlush;
        public uint favorDecSpeed;
        public fixed uint reserved[3];
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct LZ4F_cctx_s
    {
        public fixed byte _unused[1];
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct LZ4F_compressOptions_t
    {
        public uint stableSrc;
        public fixed uint reserved[3];
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct LZ4F_dctx_s
    {
        public fixed byte _unused[1];
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct LZ4F_decompressOptions_t
    {
        public uint stableDst;
        public uint skipChecksums;
        public uint reserved1;
        public uint reserved0;
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct XXH32_state_s
    {
        public fixed byte _unused[1];
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct XXH32_canonical_t
    {
        public fixed byte digest[4];
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct XXH64_state_s
    {
        public fixed byte _unused[1];
    }

    [StructLayout(LayoutKind.Sequential)]
    public unsafe struct XXH64_canonical_t
    {
        public fixed byte digest[8];
    }

    
}
    