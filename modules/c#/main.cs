using System;
using System.Collections.Generic;

enum InstructionType {
    Push,
    Add,
    Subtract,
    Dupe,
    Rotate,
    Call,
    JumpIfGreaterOrEqual,
    Return
}

class Instruction {
    public InstructionType Type { get; }
    public int Value { get; }

    public Instruction(InstructionType type, int value = 0) {
        Type = type;
        Value = value;
    }
}

class VM {
    private Stack<int> stack = new Stack<int>();
    private Stack<int> callStack = new Stack<int>();
    private int pc = 0;
    private List<Instruction> code;

    public VM(List<Instruction> code) {
        this.code = code;
    }

    public int? Run() {
        while (pc < code.Count) {
            var instr = code[pc];
            switch (instr.Type) {
                case InstructionType.Push:
                    stack.Push(instr.Value);
                    break;
                case InstructionType.Add:
                    var a = stack.Pop();
                    var b = stack.Pop();
                    stack.Push(a + b);
                    break;
                case InstructionType.Subtract:
                    a = stack.Pop();
                    b = stack.Pop();
                    stack.Push(a - b);
                    break;
                case InstructionType.Dupe:
                    stack.Push(stack.Peek());
                    break;
                case InstructionType.Rotate:
                    a = stack.Pop();
                    b = stack.Pop();
                    stack.Push(a);
                    stack.Push(b);
                    break;
                case InstructionType.Call:
                    callStack.Push(pc);
                    pc = instr.Value - 1;
                    break;
                case InstructionType.JumpIfGreaterOrEqual:
                    a = stack.Pop();
                    b = stack.Pop();
                    if (a >= b) {
                        pc = instr.Value - 1;
                    }
                    break;
                case InstructionType.Return:
                    if (callStack.Count > 0) {
                        pc = callStack.Pop();
                    } else {
                        return stack.Pop();
                    }
                    break;
            }
            pc++;
        }
        return null;
    }
}

class Program {
    public static List<Instruction> Fibonacci(int n) {
        return new List<Instruction> {
            new Instruction(InstructionType.Push, n),
            new Instruction(InstructionType.Dupe),
            new Instruction(InstructionType.Push, 1),
            new Instruction(InstructionType.JumpIfGreaterOrEqual, 15),
            new Instruction(InstructionType.Dupe),
            new Instruction(InstructionType.Push, 1),
            new Instruction(InstructionType.Rotate),
            new Instruction(InstructionType.Subtract),
            new Instruction(InstructionType.Call, 1),
            new Instruction(InstructionType.Rotate),
            new Instruction(InstructionType.Push, 2),
            new Instruction(InstructionType.Rotate),
            new Instruction(InstructionType.Subtract),
            new Instruction(InstructionType.Call, 1),
            new Instruction(InstructionType.Add),
            new Instruction(InstructionType.Return)
        };
    }

    public static int FibonacciNative(int n) {
        if (n <= 1) {
            return n;
        }
        return FibonacciNative(n - 1) + FibonacciNative(n - 2);
    }

    static void Main(string[] args) {
        if (args.Length != 1 && args.Length != 2) {
            Console.WriteLine("Usage: {0} <n> [--native]", Environment.GetCommandLineArgs()[0]);
            return;
        }

        var n = int.Parse(args[0]);
        var native = args.Length == 2 && args[1] == "--native";
    
        if (native) {
            var result = FibonacciNative(n);
            Console.WriteLine(result);
        } else {
            var code = Fibonacci(n);
            var vm = new VM(code);
            var result = vm.Run();
            Console.WriteLine(result);
        }
    }
}
