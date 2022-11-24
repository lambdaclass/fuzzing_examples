; ModuleID = 'probe1.103148df-cgu.0'
source_filename = "probe1.103148df-cgu.0"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

; core::f64::<impl f64>::to_int_unchecked
; Function Attrs: inlinehint nonlazybind uwtable
define i32 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$16to_int_unchecked17hab42b9c719e64fe9E"(double %self) unnamed_addr #0 !dbg !6 {
start:
  %self.dbg.spill = alloca double, align 8
  store double %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !16, metadata !DIExpression()), !dbg !19
; call <f64 as core::convert::num::FloatToInt<i32>>::to_int_unchecked
  %0 = call i32 @"_ZN65_$LT$f64$u20$as$u20$core..convert..num..FloatToInt$LT$i32$GT$$GT$16to_int_unchecked17h4fd7efd26c0c85cdE"(double %self), !dbg !20
  br label %bb1, !dbg !20

bb1:                                              ; preds = %start
  ret i32 %0, !dbg !21
}

; <f64 as core::convert::num::FloatToInt<i32>>::to_int_unchecked
; Function Attrs: inlinehint nonlazybind uwtable
define internal i32 @"_ZN65_$LT$f64$u20$as$u20$core..convert..num..FloatToInt$LT$i32$GT$$GT$16to_int_unchecked17h4fd7efd26c0c85cdE"(double %self) unnamed_addr #0 !dbg !22 {
start:
  %0 = alloca i32, align 4
  %self.dbg.spill = alloca double, align 8
  store double %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !28, metadata !DIExpression()), !dbg !30
  %1 = fptosi double %self to i32, !dbg !31
  store i32 %1, ptr %0, align 4, !dbg !31
  %2 = load i32, ptr %0, align 4, !dbg !31
  br label %bb1, !dbg !31

bb1:                                              ; preds = %start
  ret i32 %2, !dbg !32
}

; probe1::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe15probe17hc7615017673276a7E() unnamed_addr #1 !dbg !33 {
start:
; call core::f64::<impl f64>::to_int_unchecked
  %_1 = call i32 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$16to_int_unchecked17hab42b9c719e64fe9E"(double 1.000000e+00), !dbg !38
  br label %bb1, !dbg !38

bb1:                                              ; preds = %start
  ret void, !dbg !39
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

attributes #0 = { inlinehint nonlazybind uwtable "target-cpu"="generic" "target-features"="+outline-atomics" }
attributes #1 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+outline-atomics" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.dbg.cu = !{!4}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i32 2, !"Dwarf Version", i32 4}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !5, producer: "clang LLVM (rustc version 1.65.0 (897e37553 2022-11-02))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)
!5 = !DIFile(filename: "probe1/@/probe1.103148df-cgu.0", directory: "/root/.cargo/registry/src/github.com-1ecc6299db9ec823/num-traits-0.2.15")
!6 = distinct !DISubprogram(name: "to_int_unchecked<i32>", linkageName: "_ZN4core3f6421_$LT$impl$u20$f64$GT$16to_int_unchecked17hab42b9c719e64fe9E", scope: !8, file: !7, line: 978, type: !11, scopeLine: 978, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !17, retainedNodes: !15)
!7 = !DIFile(filename: "/rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/num/f64.rs", directory: "", checksumkind: CSK_MD5, checksum: "ab373a71f2702ec955cb63b22857bf31")
!8 = !DINamespace(name: "{impl#0}", scope: !9)
!9 = !DINamespace(name: "f64", scope: !10)
!10 = !DINamespace(name: "core", scope: null)
!11 = !DISubroutineType(types: !12)
!12 = !{!13, !14}
!13 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!14 = !DIBasicType(name: "f64", size: 64, encoding: DW_ATE_float)
!15 = !{!16}
!16 = !DILocalVariable(name: "self", arg: 1, scope: !6, file: !7, line: 978, type: !14)
!17 = !{!18}
!18 = !DITemplateTypeParameter(name: "Int", type: !13)
!19 = !DILocation(line: 978, column: 41, scope: !6)
!20 = !DILocation(line: 984, column: 18, scope: !6)
!21 = !DILocation(line: 985, column: 6, scope: !6)
!22 = distinct !DISubprogram(name: "to_int_unchecked", linkageName: "_ZN65_$LT$f64$u20$as$u20$core..convert..num..FloatToInt$LT$i32$GT$$GT$16to_int_unchecked17h4fd7efd26c0c85cdE", scope: !24, file: !23, line: 29, type: !11, scopeLine: 29, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !29, retainedNodes: !27)
!23 = !DIFile(filename: "/rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/convert/num.rs", directory: "", checksumkind: CSK_MD5, checksum: "e91066e40000f1322cc7cfcb428337d9")
!24 = !DINamespace(name: "{impl#22}", scope: !25)
!25 = !DINamespace(name: "num", scope: !26)
!26 = !DINamespace(name: "convert", scope: !10)
!27 = !{!28}
!28 = !DILocalVariable(name: "self", arg: 1, scope: !22, file: !23, line: 29, type: !14)
!29 = !{}
!30 = !DILocation(line: 29, column: 44, scope: !22)
!31 = !DILocation(line: 31, column: 30, scope: !22)
!32 = !DILocation(line: 32, column: 18, scope: !22)
!33 = distinct !DISubprogram(name: "probe", linkageName: "_ZN6probe15probe17hc7615017673276a7E", scope: !35, file: !34, line: 1, type: !36, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !4, templateParams: !29, retainedNodes: !29)
!34 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_MD5, checksum: "b691dbdf770db888ec6eb1194ed0df31")
!35 = !DINamespace(name: "probe1", scope: null)
!36 = !DISubroutineType(types: !37)
!37 = !{null}
!38 = !DILocation(line: 1, column: 35, scope: !33)
!39 = !DILocation(line: 1, column: 70, scope: !40)
!40 = !DILexicalBlockFile(scope: !33, file: !34, discriminator: 0)
