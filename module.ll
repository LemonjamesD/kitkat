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

body_for:                                         ; preds = %escape_if7, %entry
  %changed = alloca i8, align 1                   ; [#uses=4 type=ptr]
  store i8 0, ptr %changed, align 1
  %loaded_value = load i8, ptr %i, align 1        ; [#uses=1 type=i8]
  %moduloed_value = urem i8 %loaded_value, 3      ; [#uses=1 type=i8]
  %equaled_value = icmp eq i8 %moduloed_value, 0  ; [#uses=1 type=i1]
  br i1 %equaled_value, label %then, label %escape_if

escape_for:                                       ; preds = %escape_if7
  ret void

then:                                             ; preds = %body_for
  call void @print_fizz()
  store i8 1, ptr %changed, align 1
  br label %escape_if

escape_if:                                        ; preds = %then, %body_for
  %loaded_value3 = load i8, ptr %i, align 1       ; [#uses=1 type=i8]
  %moduloed_value4 = urem i8 %loaded_value3, 5    ; [#uses=1 type=i8]
  %equaled_value5 = icmp eq i8 %moduloed_value4, 0 ; [#uses=1 type=i1]
  br i1 %equaled_value5, label %then1, label %escape_if2

then1:                                            ; preds = %escape_if
  call void @print_buzz()
  store i8 1, ptr %changed, align 1
  br label %escape_if2

escape_if2:                                       ; preds = %then1, %escape_if
  %loaded_value8 = load i8, ptr %changed, align 1 ; [#uses=1 type=i8]
  %equaled_value9 = icmp eq i8 %loaded_value8, 0  ; [#uses=1 type=i1]
  br i1 %equaled_value9, label %then6, label %escape_if7

then6:                                            ; preds = %escape_if2
  %loaded_value10 = load i8, ptr %i, align 1      ; [#uses=1 type=i8]
  call void @print_int(i8 %loaded_value10)
  br label %escape_if7

escape_if7:                                       ; preds = %then6, %escape_if2
  call void @new_line()
  %loaded_value11 = load i8, ptr %i, align 1      ; [#uses=1 type=i8]
  %added_value = add i8 %loaded_value11, 1        ; [#uses=1 type=i8]
  store i8 %added_value, ptr %i, align 1
  %added_value12 = add i8 %1, 1                   ; [#uses=1 type=i8]
  %loaded_value13 = load i8, ptr %i, align 1      ; [#uses=1 type=i8]
  %gt_value = icmp ugt i8 %added_value12, %loaded_value13 ; [#uses=1 type=i1]
  br i1 %gt_value, label %body_for, label %escape_for
}

; [#uses=0]
define i8 @main() {
entry:
  call void @fizz_buzz(i8 1, i8 100)
  ret i8 0
}
