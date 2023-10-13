; ModuleID = 'module.bc'
source_filename = "main"

declare void @print_int(i8)

define i8 @cool(i8 %0) {
entry:
  %equaled_value = icmp eq i8 %0, 10
  br i1 %equaled_value, label %then, label %else

then:                                             ; preds = %entry
  %added_value = add i8 10, %0
  ret i8 %added_value

else:                                             ; preds = %entry
  ret i8 %0
}

define i8 @main() {
entry:
  %i = alloca i8, align 1
  store i8 1, ptr %i, align 1
  br label %body_for

body_for:                                         ; preds = %body_for, %entry
  %loaded_value = load i8, ptr %i, align 1
  call void @print_int(i8 %loaded_value)
  %loaded_value1 = load i8, ptr %i, align 1
  %added_value = add i8 %loaded_value1, 1
  store i8 %added_value, ptr %i, align 1
  %loaded_value2 = load i8, ptr %i, align 1
  %gt_value = icmp ugt i8 10, %loaded_value2
  br i1 %gt_value, label %body_for, label %escape_for

escape_for:                                       ; preds = %body_for
  %loaded_value3 = load i8, ptr %i, align 1
  ret i8 %loaded_value3
}
