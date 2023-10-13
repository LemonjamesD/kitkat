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
  store i8 30, ptr %x, align 1
  %loaded_value = load i8, ptr %x, align 1
  %cool = call i8 @cool(i8 %loaded_value)
  ret i8 %cool
}
