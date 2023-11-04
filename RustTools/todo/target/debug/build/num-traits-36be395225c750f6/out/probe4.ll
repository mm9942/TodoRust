; ModuleID = 'probe4.4f301258ec8fe861-cgu.0'
source_filename = "probe4.4f301258ec8fe861-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-apple-macosx10.7.0"

@alloc_5a8593967b8972866a7e1ecddfeddfa5 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/fe7454bf439c93cbe9ac8a8f7fcfacd5a40244c2/library/core/src/num/mod.rs" }>, align 1
@alloc_e34b87e5b900304445d7a9a7a04b89fd = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5a8593967b8972866a7e1ecddfeddfa5, [16 x i8] c"K\00\00\00\00\00\00\00w\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: uwtable
define void @_ZN6probe45probe17hcc74fa27ef097762E() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7873548f5ce70213E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h3070a0df3233a4b2E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_e34b87e5b900304445d7a9a7a04b89fd) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h7873548f5ce70213E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h3070a0df3233a4b2E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { uwtable "frame-pointer"="all" "probe-stack"="inline-asm" "target-cpu"="core2" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="all" "probe-stack"="inline-asm" "target-cpu"="core2" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
