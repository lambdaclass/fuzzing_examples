; ModuleID = 'probe5.f2dbdd71-cgu.0'
source_filename = "probe5.f2dbdd71-cgu.0"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

@alloc3 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/num/mod.rs" }>, align 1
@alloc4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc3, [16 x i8] c"K\00\00\00\00\00\00\00Z\03\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe5::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe55probe17h1fc3bc572dc2ac25E() unnamed_addr #0 !dbg !6 {
start:
  %rhs.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca i32, align 4
  store i32 1, ptr %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill.i, metadata !12, metadata !DIExpression()), !dbg !23
  store i32 1, ptr %rhs.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata ptr %rhs.dbg.spill.i, metadata !22, metadata !DIExpression()), !dbg !25
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false), !dbg !26
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6e828580a623dbaaE.exit", !dbg !26

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17ha983d1ee14be5ecbE(ptr align 1 @str.0, i64 25, ptr align 8 @alloc4) #4, !dbg !26
  unreachable, !dbg !26

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6e828580a623dbaaE.exit": ; preds = %start
  br label %bb1, !dbg !27

bb1:                                              ; preds = %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6e828580a623dbaaE.exit"
  ret void, !dbg !28
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #2

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17ha983d1ee14be5ecbE(ptr align 1, i64, ptr align 8) unnamed_addr #3

attributes #0 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+outline-atomics" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #3 = { cold noinline noreturn nonlazybind uwtable "target-cpu"="generic" "target-features"="+outline-atomics" }
attributes #4 = { noreturn }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.dbg.cu = !{!4}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i32 2, !"Dwarf Version", i32 4}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !5, producer: "clang LLVM (rustc version 1.65.0 (897e37553 2022-11-02))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)
!5 = !DIFile(filename: "probe5/@/probe5.f2dbdd71-cgu.0", directory: "/root/.cargo/registry/src/github.com-1ecc6299db9ec823/num-traits-0.2.15")
!6 = distinct !DISubprogram(name: "probe", linkageName: "_ZN6probe55probe17h1fc3bc572dc2ac25E", scope: !8, file: !7, line: 1, type: !9, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !4, templateParams: !11, retainedNodes: !11)
!7 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_MD5, checksum: "ba5543a6264010df82a4e051311d5726")
!8 = !DINamespace(name: "probe5", scope: null)
!9 = !DISubroutineType(types: !10)
!10 = !{null}
!11 = !{}
!12 = !DILocalVariable(name: "self", arg: 1, scope: !13, file: !14, line: 1991, type: !20)
!13 = distinct !DISubprogram(name: "div_euclid", linkageName: "_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h6e828580a623dbaaE", scope: !15, file: !14, line: 1991, type: !18, scopeLine: 1991, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !11, retainedNodes: !21)
!14 = !DIFile(filename: "/rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/num/uint_macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "3ae2f8aa990e4bb4ebe69ab6d8616c00")
!15 = !DINamespace(name: "{impl#9}", scope: !16)
!16 = !DINamespace(name: "num", scope: !17)
!17 = !DINamespace(name: "core", scope: null)
!18 = !DISubroutineType(types: !19)
!19 = !{!20, !20, !20}
!20 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!21 = !{!12, !22}
!22 = !DILocalVariable(name: "rhs", arg: 2, scope: !13, file: !14, line: 1991, type: !20)
!23 = !DILocation(line: 1991, column: 33, scope: !13, inlinedAt: !24)
!24 = distinct !DILocation(line: 1, column: 26, scope: !6)
!25 = !DILocation(line: 1991, column: 39, scope: !13, inlinedAt: !24)
!26 = !DILocation(line: 1992, column: 13, scope: !13, inlinedAt: !24)
!27 = !DILocation(line: 1, column: 26, scope: !6)
!28 = !DILocation(line: 1, column: 50, scope: !29)
!29 = !DILexicalBlockFile(scope: !6, file: !7, discriminator: 0)
