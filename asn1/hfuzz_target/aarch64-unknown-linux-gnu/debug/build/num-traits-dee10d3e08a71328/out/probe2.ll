; ModuleID = 'probe2.6109820e-cgu.0'
source_filename = "probe2.6109820e-cgu.0"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

; probe2::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe25probe17hac59d0d8b58b64caE() unnamed_addr #0 !dbg !6 {
start:
  %0 = alloca i32, align 4
  %self.dbg.spill.i = alloca i32, align 4
  store i32 1, ptr %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill.i, metadata !12, metadata !DIExpression()), !dbg !22
  store i32 -2147483648, ptr %0, align 4, !dbg !24
  %1 = load i32, ptr %0, align 4, !dbg !24
  br label %bb1, !dbg !25

bb1:                                              ; preds = %start
  ret void, !dbg !26
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.bitreverse.i32(i32) #1

attributes #0 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+outline-atomics" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.dbg.cu = !{!4}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i32 2, !"Dwarf Version", i32 4}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !5, producer: "clang LLVM (rustc version 1.65.0 (897e37553 2022-11-02))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)
!5 = !DIFile(filename: "probe2/@/probe2.6109820e-cgu.0", directory: "/root/.cargo/registry/src/github.com-1ecc6299db9ec823/num-traits-0.2.15")
!6 = distinct !DISubprogram(name: "probe", linkageName: "_ZN6probe25probe17hac59d0d8b58b64caE", scope: !8, file: !7, line: 1, type: !9, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !4, templateParams: !11, retainedNodes: !11)
!7 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_MD5, checksum: "7e675ce338b2f129514ed2c095ce6ce1")
!8 = !DINamespace(name: "probe2", scope: null)
!9 = !DISubroutineType(types: !10)
!10 = !{null}
!11 = !{}
!12 = !DILocalVariable(name: "self", arg: 1, scope: !13, file: !14, line: 281, type: !20)
!13 = distinct !DISubprogram(name: "reverse_bits", linkageName: "_ZN4core3num21_$LT$impl$u20$u32$GT$12reverse_bits17h7288ad1c2340f953E", scope: !15, file: !14, line: 281, type: !18, scopeLine: 281, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !11, retainedNodes: !21)
!14 = !DIFile(filename: "/rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/num/uint_macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "3ae2f8aa990e4bb4ebe69ab6d8616c00")
!15 = !DINamespace(name: "{impl#9}", scope: !16)
!16 = !DINamespace(name: "num", scope: !17)
!17 = !DINamespace(name: "core", scope: null)
!18 = !DISubroutineType(types: !19)
!19 = !{!20, !20}
!20 = !DIBasicType(name: "u32", size: 32, encoding: DW_ATE_unsigned)
!21 = !{!12}
!22 = !DILocation(line: 281, column: 35, scope: !13, inlinedAt: !23)
!23 = distinct !DILocation(line: 1, column: 26, scope: !6)
!24 = !DILocation(line: 282, column: 13, scope: !13, inlinedAt: !23)
!25 = !DILocation(line: 1, column: 26, scope: !6)
!26 = !DILocation(line: 1, column: 48, scope: !27)
!27 = !DILexicalBlockFile(scope: !6, file: !7, discriminator: 0)
