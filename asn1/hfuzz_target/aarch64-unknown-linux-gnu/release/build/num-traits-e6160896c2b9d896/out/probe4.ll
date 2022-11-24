; ModuleID = 'probe4.ae85e1df-cgu.0'
source_filename = "probe4.ae85e1df-cgu.0"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

$_ZN6probe45probe17h83bfbc1443a555f8E = comdat nodeduplicate

$sancov.module_ctor_trace_pc_guard = comdat any

@__sancov_lowest_stack = external thread_local(initialexec) global i64
@__sancov_gen_ = private global [1 x i32] zeroinitializer, section "__sancov_guards", comdat($_ZN6probe45probe17h83bfbc1443a555f8E), align 4
@__start___sancov_guards = extern_weak hidden global i32
@__stop___sancov_guards = extern_weak hidden global i32
@llvm.global_ctors = appending global [1 x { i32, ptr, ptr }] [{ i32, ptr, ptr } { i32 2, ptr @sancov.module_ctor_trace_pc_guard, ptr @sancov.module_ctor_trace_pc_guard }]
@llvm.used = appending global [1 x ptr] [ptr @sancov.module_ctor_trace_pc_guard], section "llvm.metadata"
@llvm.compiler.used = appending global [1 x ptr] [ptr @__sancov_gen_], section "llvm.metadata"

; probe4::probe
; Function Attrs: mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn uwtable
define void @_ZN6probe45probe17h83bfbc1443a555f8E() unnamed_addr #0 comdat {
start:
  call void @__sanitizer_cov_trace_pc_guard(ptr @__sancov_gen_) #2
  ret void
}

declare void @__sanitizer_cov_trace_pc_indir(i64)

declare void @__sanitizer_cov_trace_cmp1(i8 zeroext, i8 zeroext)

declare void @__sanitizer_cov_trace_cmp2(i16 zeroext, i16 zeroext)

declare void @__sanitizer_cov_trace_cmp4(i32 zeroext, i32 zeroext)

declare void @__sanitizer_cov_trace_cmp8(i64, i64)

declare void @__sanitizer_cov_trace_const_cmp1(i8 zeroext, i8 zeroext)

declare void @__sanitizer_cov_trace_const_cmp2(i16 zeroext, i16 zeroext)

declare void @__sanitizer_cov_trace_const_cmp4(i32 zeroext, i32 zeroext)

declare void @__sanitizer_cov_trace_const_cmp8(i64, i64)

declare void @__sanitizer_cov_load1(ptr)

declare void @__sanitizer_cov_load2(ptr)

declare void @__sanitizer_cov_load4(ptr)

declare void @__sanitizer_cov_load8(ptr)

declare void @__sanitizer_cov_load16(ptr)

declare void @__sanitizer_cov_store1(ptr)

declare void @__sanitizer_cov_store2(ptr)

declare void @__sanitizer_cov_store4(ptr)

declare void @__sanitizer_cov_store8(ptr)

declare void @__sanitizer_cov_store16(ptr)

declare void @__sanitizer_cov_trace_div4(i32 zeroext)

declare void @__sanitizer_cov_trace_div8(i64)

declare void @__sanitizer_cov_trace_gep(i64)

declare void @__sanitizer_cov_trace_switch(i64, ptr)

declare void @__sanitizer_cov_trace_pc()

declare void @__sanitizer_cov_trace_pc_guard(ptr)

declare void @__sanitizer_cov_trace_pc_guard_init(ptr, ptr)

; Function Attrs: nounwind
define internal void @sancov.module_ctor_trace_pc_guard() #1 comdat {
  call void @__sanitizer_cov_trace_pc_guard_init(ptr @__start___sancov_guards, ptr @__stop___sancov_guards)
  ret void
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind nonlazybind readnone willreturn uwtable "target-cpu"="generic" "target-features"="+neon,+fp-armv8,+crypto,+lse,+crc,+outline-atomics" }
attributes #1 = { nounwind }
attributes #2 = { nomerge }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
