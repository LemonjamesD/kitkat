; ModuleID = 'module.bc'
source_filename = "main"

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
  %x = alloca i8, align 1
  store i8 10, ptr %x, align 1
  br label %init_for

init_for:                                         ; preds = %entry
  %i = alloca i8, align 1
  store i8 1, ptr %i, align 1
  br label %end_for

end_for:                                          ; preds = %end_for, %init_for
  %loaded_value = load i8, ptr %x, align 1
  %added_value = add i8 %loaded_value, 1
  store i8 %added_value, ptr %x, align 1
  %loaded_value1 = load i8, ptr %i, align 1
  %added_value2 = add i8 %loaded_value1, 1
  store i8 %added_value2, ptr %i, align 1
  %loaded_value3 = load i8, ptr %i, align 1
  %gt_value = icmp ugt i8 10, %loaded_value3
  br i1 %gt_value, label %end_for, label %escape_for

escape_for:                                       ; preds = %end_for
  %loaded_value4 = load i8, ptr %x, align 1
  %cool = call i8 @cool(i8 %loaded_value4)
  ret i8 %cool
}
