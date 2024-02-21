; ModuleID = 'probe5.ec68d346db31f639-cgu.0'
source_filename = "probe5.ec68d346db31f639-cgu.0"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; core::f64::<impl f64>::is_subnormal
; Function Attrs: inlinehint nonlazybind uwtable
define internal zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h46199f5fb45f738cE"(double %self) unnamed_addr #0 {
start:
  %_2 = alloca i8, align 1
; call core::f64::<impl f64>::classify
  %0 = call i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17haf569e9bc9a66f4cE"(double %self), !range !3
  store i8 %0, ptr %_2, align 1
  %1 = load i8, ptr %_2, align 1, !range !3, !noundef !4
  %_3 = zext i8 %1 to i64
  %_0 = icmp eq i64 %_3, 3
  ret i1 %_0
}

; probe5::probe
; Function Attrs: nonlazybind uwtable
define void @_ZN6probe55probe17h85db602ef3d40995E() unnamed_addr #1 {
start:
; call core::f64::<impl f64>::is_subnormal
  %_1 = call zeroext i1 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$12is_subnormal17h46199f5fb45f738cE"(double 1.000000e+00)
  ret void
}

; core::f64::<impl f64>::classify
; Function Attrs: nonlazybind uwtable
declare i8 @"_ZN4core3f6421_$LT$impl$u20$f64$GT$8classify17haf569e9bc9a66f4cE"(double) unnamed_addr #1

attributes #0 = { inlinehint nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }
attributes #1 = { nonlazybind uwtable "probe-stack"="inline-asm" "target-cpu"="x86-64" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
!2 = !{!"rustc version 1.73.0 (cc66ad468 2023-10-03) (built from a source tarball)"}
!3 = !{i8 0, i8 5}
!4 = !{}
