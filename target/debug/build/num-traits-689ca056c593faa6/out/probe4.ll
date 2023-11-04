; ModuleID = 'probe4.cf380da87c921c41-cgu.0'
source_filename = "probe4.cf380da87c921c41-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@alloc_21ea156a8fd112cbea267cc512db7b10 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/a2f5f9691b6ce64c1703feaf9363710dfd7a56cf/library/core/src/num/mod.rs" }>, align 1
@alloc_b1dc2ae45565d6368bc453498553cee6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_21ea156a8fd112cbea267cc512db7b10, [16 x i8] c"K\00\00\00\00\00\00\00y\04\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"

; probe4::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe45probe17h67e2c14cd8f37eafE() unnamed_addr #0 {
start:
  %0 = call i1 @llvm.expect.i1(i1 false, i1 false)
  br i1 %0, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h39f134a7148bf7a4E.exit"

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17hdbe38cfede3323f7E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc_b1dc2ae45565d6368bc453498553cee6) #3
  unreachable

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h39f134a7148bf7a4E.exit": ; preds = %start
  ret void
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(none)
declare i1 @llvm.expect.i1(i1, i1) #1

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17hdbe38cfede3323f7E(ptr align 1, i64, ptr align 8) unnamed_addr #2

attributes #0 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind willreturn memory(none) }
attributes #2 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.75.0-nightly (a2f5f9691 2023-11-02)"}
