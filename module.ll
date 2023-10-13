; ModuleID = 'module.bc'
source_filename = "main"

; [#uses=1]
declare void @print_int(i8)

; [#uses=1]
declare void @new_line()

; [#uses=1]
declare void @print_fizz()

; [#uses=1]
declare void @print_buzz()

; [#uses=1]
define void @fizz_buzz(i8 %0, i8 %1) {
entry:
  %i = alloca i8, align 1                         ; [#uses=7 type=ptr]
  store i8 %0, ptr %i, align 1
  br label %body_for

body_for:                                         ; preds = %escape_if, %entry
  %loaded_value = load i8, ptr %i, align 1        ; [#uses=1 type=i8]
  %moduloed_value = urem i8 %loaded_value, 5      ; [#uses=1 type=i8]
  %equaled_value = icmp eq i8 %moduloed_value, 0  ; [#uses=1 type=i1]
  br i1 %equaled_value, label %then, label %else

escape_for:                                       ; preds = %escape_if
  ret void

then:                                             ; preds = %body_for
  call void @print_fizz()
  br label %escape_if

else:                                             ; preds = %body_for
  %loaded_value4 = load i8, ptr %i, align 1       ; [#uses=1 type=i8]
  %moduloed_value5 = urem i8 %loaded_value4, 3    ; [#uses=1 type=i8]
  %equaled_value6 = icmp eq i8 %moduloed_value5, 0 ; [#uses=1 type=i1]
  br i1 %equaled_value6, label %then1, label %else2

escape_if:                                        ; preds = %escape_if3, %then
  call void @new_line()
  %loaded_value8 = load i8, ptr %i, align 1       ; [#uses=1 type=i8]
  %added_value = add i8 %loaded_value8, 1         ; [#uses=1 type=i8]
  store i8 %added_value, ptr %i, align 1
  %loaded_value9 = load i8, ptr %i, align 1       ; [#uses=1 type=i8]
  %gt_value = icmp ugt i8 %1, %loaded_value9      ; [#uses=1 type=i1]
  br i1 %gt_value, label %body_for, label %escape_for

then1:                                            ; preds = %else
  call void @print_buzz()
  br label %escape_if3

else2:                                            ; preds = %else
  %loaded_value7 = load i8, ptr %i, align 1       ; [#uses=1 type=i8]
  call void @print_int(i8 %loaded_value7)
  br label %escape_if3

escape_if3:                                       ; preds = %else2, %then1
  br label %escape_if
}

; [#uses=0]
define i8 @main() {
entry:
  call void @fizz_buzz(i8 0, i8 10)
  ret i8 0
}
