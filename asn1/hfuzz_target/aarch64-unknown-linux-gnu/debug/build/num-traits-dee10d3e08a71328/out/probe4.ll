; ModuleID = 'probe4.ae85e1df-cgu.0'
source_filename = "probe4.ae85e1df-cgu.0"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

@alloc5 = private unnamed_addr constant <{ [77 x i8] }> <{ [77 x i8] c"/rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/arith.rs" }>, align 1
@alloc6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc5, [16 x i8] c"M\00\00\00\00\00\00\00\12\03\00\00\01\00\00\00" }>, align 8
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc3 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"\02\00\00\00" }>, align 4

; <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
; Function Attrs: inlinehint nonlazybind uwtable
define internal void @"_ZN66_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$LT$$RF$i32$GT$$GT$10add_assign17h2b07db545bbf48aeE"(ptr align 4 %self, ptr align 4 %other) unnamed_addr #0 !dbg !6 {
start:
  %other.dbg.spill3 = alloca i32, align 4
  %self.dbg.spill1 = alloca ptr, align 8
  %other.dbg.spill = alloca ptr, align 8
  %self.dbg.spill = alloca ptr, align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !18, metadata !DIExpression()), !dbg !21
  store ptr %other, ptr %other.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %other.dbg.spill, metadata !19, metadata !DIExpression()), !dbg !22
  store ptr %self, ptr %self.dbg.spill1, align 8, !dbg !23
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill1, metadata !24, metadata !DIExpression()), !dbg !32
  %other2 = load i32, ptr %other, align 4, !dbg !34
  store i32 %other2, ptr %other.dbg.spill3, align 4, !dbg !34
  call void @llvm.dbg.declare(metadata ptr %other.dbg.spill3, metadata !31, metadata !DIExpression()), !dbg !35
  %0 = load i32, ptr %self, align 4, !dbg !36
  %1 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %0, i32 %other2), !dbg !36
  %_6.0 = extractvalue { i32, i1 } %1, 0, !dbg !36
  %_6.1 = extractvalue { i32, i1 } %1, 1, !dbg !36
  %2 = call i1 @llvm.expect.i1(i1 %_6.1, i1 false), !dbg !36
  br i1 %2, label %panic, label %bb1, !dbg !36

bb1:                                              ; preds = %start
  store i32 %_6.0, ptr %self, align 4, !dbg !36
  ret void, !dbg !37

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17ha983d1ee14be5ecbE(ptr align 1 @str.0, i64 28, ptr align 8 @alloc6) #5, !dbg !36
  unreachable, !dbg !36
}

; probe4::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe45probe17h83bfbc1443a555f8E() unnamed_addr #1 !dbg !38 {
start:
  %x = alloca i32, align 4
  call void @llvm.dbg.declare(metadata ptr %x, metadata !44, metadata !DIExpression()), !dbg !46
  store i32 1, ptr %x, align 4, !dbg !47
; call <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
  call void @"_ZN66_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$LT$$RF$i32$GT$$GT$10add_assign17h2b07db545bbf48aeE"(ptr align 4 %x, ptr align 4 @alloc3), !dbg !48
  br label %bb1, !dbg !48

bb1:                                              ; preds = %start
  ret void, !dbg !49
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17ha983d1ee14be5ecbE(ptr align 1, i64, ptr align 8) unnamed_addr #4

attributes #0 = { inlinehint nonlazybind uwtable "target-cpu"="generic" "target-features"="+outline-atomics" }
attributes #1 = { nonlazybind uwtable "target-cpu"="generic" "target-features"="+outline-atomics" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #4 = { cold noinline noreturn nonlazybind uwtable "target-cpu"="generic" "target-features"="+outline-atomics" }
attributes #5 = { noreturn }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.dbg.cu = !{!4}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{i32 2, !"Dwarf Version", i32 4}
!3 = !{i32 2, !"Debug Info Version", i32 3}
!4 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !5, producer: "clang LLVM (rustc version 1.65.0 (897e37553 2022-11-02))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug)
!5 = !DIFile(filename: "probe4/@/probe4.ae85e1df-cgu.0", directory: "/root/.cargo/registry/src/github.com-1ecc6299db9ec823/num-traits-0.2.15")
!6 = distinct !DISubprogram(name: "add_assign", linkageName: "_ZN66_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$LT$$RF$i32$GT$$GT$10add_assign17h2b07db545bbf48aeE", scope: !8, file: !7, line: 126, type: !12, scopeLine: 126, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !20, retainedNodes: !17)
!7 = !DIFile(filename: "/rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/internal_macros.rs", directory: "", checksumkind: CSK_MD5, checksum: "e63771959055ac6eeea73519b0063b2f")
!8 = !DINamespace(name: "{impl#319}", scope: !9)
!9 = !DINamespace(name: "arith", scope: !10)
!10 = !DINamespace(name: "ops", scope: !11)
!11 = !DINamespace(name: "core", scope: null)
!12 = !DISubroutineType(types: !13)
!13 = !{null, !14, !16}
!14 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&mut i32", baseType: !15, size: 64, align: 64, dwarfAddressSpace: 0)
!15 = !DIBasicType(name: "i32", size: 32, encoding: DW_ATE_signed)
!16 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&i32", baseType: !15, size: 64, align: 64, dwarfAddressSpace: 0)
!17 = !{!18, !19}
!18 = !DILocalVariable(name: "self", arg: 1, scope: !6, file: !7, line: 126, type: !14)
!19 = !DILocalVariable(name: "other", arg: 2, scope: !6, file: !7, line: 126, type: !16)
!20 = !{}
!21 = !DILocation(line: 126, column: 24, scope: !6)
!22 = !DILocation(line: 126, column: 35, scope: !6)
!23 = !DILocation(line: 127, column: 31, scope: !6)
!24 = !DILocalVariable(name: "self", scope: !25, file: !26, line: 779, type: !14, align: 8)
!25 = distinct !DISubprogram(name: "add_assign", linkageName: "_ZN51_$LT$i32$u20$as$u20$core..ops..arith..AddAssign$GT$10add_assign17h27f51aea2d996ad0E", scope: !27, file: !26, line: 779, type: !28, scopeLine: 779, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !4, templateParams: !20, retainedNodes: !30)
!26 = !DIFile(filename: "/rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/arith.rs", directory: "", checksumkind: CSK_MD5, checksum: "57699c352beef63103a9f45527b1cedb")
!27 = !DINamespace(name: "{impl#305}", scope: !9)
!28 = !DISubroutineType(types: !29)
!29 = !{null, !14, !15}
!30 = !{!24, !31}
!31 = !DILocalVariable(name: "other", scope: !25, file: !26, line: 779, type: !15, align: 4)
!32 = !DILocation(line: 779, column: 27, scope: !25, inlinedAt: !33)
!33 = !DILocation(line: 127, column: 17, scope: !6)
!34 = !DILocation(line: 127, column: 37, scope: !6)
!35 = !DILocation(line: 779, column: 38, scope: !25, inlinedAt: !33)
!36 = !DILocation(line: 779, column: 51, scope: !25, inlinedAt: !33)
!37 = !DILocation(line: 128, column: 14, scope: !6)
!38 = distinct !DISubprogram(name: "probe", linkageName: "_ZN6probe45probe17h83bfbc1443a555f8E", scope: !40, file: !39, line: 1, type: !41, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !4, templateParams: !20, retainedNodes: !43)
!39 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_MD5, checksum: "c9f62eb2fe042f920d6370c2956f3e5e")
!40 = !DINamespace(name: "probe4", scope: null)
!41 = !DISubroutineType(types: !42)
!42 = !{null}
!43 = !{!44}
!44 = !DILocalVariable(name: "x", scope: !45, file: !39, line: 1, type: !15, align: 4)
!45 = distinct !DILexicalBlock(scope: !38, file: !39, line: 1, column: 28)
!46 = !DILocation(line: 1, column: 32, scope: !45)
!47 = !DILocation(line: 1, column: 40, scope: !38)
!48 = !DILocation(line: 1, column: 43, scope: !45)
!49 = !DILocation(line: 1, column: 56, scope: !50)
!50 = !DILexicalBlockFile(scope: !38, file: !39, discriminator: 0)
