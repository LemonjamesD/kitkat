; ModuleID = 'module.bc'
source_filename = "main"

declare void @print_fizz()

declare void @print_buzz()

define void @fizz_buzz(i8 %0, i8 %1) {
entry:
  %i = alloca i8, align 1
  store i8 %0, ptr %i, align 1
  br label %body_for

body_for:                                         ; preds = %body_for, %entry
  %loaded_value = load i8, ptr %i, align 1
  %added_value = add i8 %loaded_value, 1
  store i8 %added_value, ptr %i, align 1
  %loaded_value1 = load i8, ptr %i, align 1
  %gt_value = icmp ugt i8 %1, %loaded_value1
  br i1 %gt_value, label %body_for, label %escape_for

escape_for:                                       ; preds = %body_for
  ret void
}

define i8 @main() {
entry:
  call void @fizz_buzz(i8 10, i8 0)
  ret i8 0
}
