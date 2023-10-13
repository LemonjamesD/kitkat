; ModuleID = 'module.bc'
source_filename = "main"

define i8 @cool(i8 %0) {
entry:
  %added_value = add i8 10, %0
  ret i8 %added_value
}

define i8 @main() {
entry:
  %cool = call i8 @cool(i8 2)
  %multiplied_value = mul i8 1, %cool
  %subbed_value = sub i8 5, %multiplied_value
  ret i8 %subbed_value
}
