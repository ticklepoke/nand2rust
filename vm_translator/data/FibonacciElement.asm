@256
D=A
@SP
M=D
(Main.fibonacci)
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@2
D=A
@SP
A=M
M=D
@SP
M=M+1
@RUN0
0;JMP
(TRUE0)
@SP
A=M
M=-1
@EQ0
0;JMP
(RUN0)
@SP
AM=M-1
D=M
M=0
@SP
 AM=M-1
D=D-M
M=0
@TRUE0
D;JGT
@SP
A=M
M=0
(EQ0)
@SP
M=M+1
@SP
AM=M-1
D=M
M=0
@IF_TRUE

D;JNE
@IF_FALSE
0;JMP
(IF_TRUE)
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@FRAME
M=D
@5
D=D-A
A=D
D=M
@RETURN
M=D
@SP
A=M-1
D=M
@ARG
A=M
M=D
@ARG
D=M
@SP
M=D+1
@FRAME
AM=M-1
D=M
@THAT
M=D
@FRAME
AM=M-1
D=M
@THIS
M=D
@FRAME
AM=M-1
D=M
@ARG
M=D
@FRAME
AM=M-1
D=M
@LCL
M=D
@RETURN
A=M
0;JMP
(IF_FALSE)
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@2
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
M=0
@R5
M=D
@SP
AM=M-1
D=M
@R5
D=D-M
@SP
A=M
M=D
@SP
M=M+1
@R5
M=0
@Return_FUNC_X
@Main.fibonacci_1
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@1
D=D-A
@5
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@_FUNC
0;JMP
(Main.fibonacci_1)
@0
D=A
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
AM=M-1
D=M
M=0
@R5
M=D
@SP
AM=M-1
D=M
@R5
D=D-M
@SP
A=M
M=D
@SP
M=M+1
@R5
M=0
@Return_FUNC_X
@Main.fibonacci_2
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@1
D=D-A
@5
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@_FUNC
0;JMP
(Main.fibonacci_2)
@SP
AM=M-1
D=M
M=0
@SP
AM=M-1
M=D+M
@SP
M=M+1
@LCL
D=M
@FRAME
M=D
@5
D=D-A
A=D
D=M
@RETURN
M=D
@SP
A=M-1
D=M
@ARG
A=M
M=D
@ARG
D=M
@SP
M=D+1
@FRAME
AM=M-1
D=M
@THAT
M=D
@FRAME
AM=M-1
D=M
@THIS
M=D
@FRAME
AM=M-1
D=M
@ARG
M=D
@FRAME
AM=M-1
D=M
@LCL
M=D
@RETURN
A=M
0;JMP
(Sys.init)
@4
D=A
@SP
A=M
M=D
@SP
M=M+1
@Return_FUNC_X
@Main.fibonacci_3
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@1
D=D-A
@5
D=D-A
@ARG
M=D
@SP
D=M
@LCL
M=D
@_FUNC
0;JMP
(Main.fibonacci_3)
(WHILE)
@WHILE
0;JMP
(END)
@END
0;JMP
