class InstructionType:
    PUSH = 'Push'
    ADD = 'Add'
    SUBTRACT = 'Subtract'
    DUPE = 'Dupe'
    ROTATE = 'Rotate'
    CALL = 'Call'
    JUMP_IF_GREATER_OR_EQUAL = 'JumpIfGreaterOrEqual'
    RETURN = 'Return'

class Instruction:
    def __init__(self, type, value=0):
        self.type = type
        self.value = value

class VM:
    def __init__(self, code):
        self.stack = []
        self.call_stack = []
        self.pc = 0
        self.code = code

    def run(self):
        while self.pc < len(self.code):
            instr = self.code[self.pc]
            if instr.type == InstructionType.PUSH:
                self.stack.append(instr.value)
            elif instr.type == InstructionType.ADD:
                a = self.stack.pop()
                b = self.stack.pop()
                self.stack.append(a + b)
            elif instr.type == InstructionType.SUBTRACT:
                a = self.stack.pop()
                b = self.stack.pop()
                self.stack.append(a - b)
            elif instr.type == InstructionType.DUPE:
                self.stack.append(self.stack[-1])
            elif instr.type == InstructionType.ROTATE:
                a = self.stack.pop()
                b = self.stack.pop()
                self.stack.append(a)
                self.stack.append(b)
            elif instr.type == InstructionType.CALL:
                self.call_stack.append(self.pc)
                self.pc = instr.value - 1
            elif instr.type == InstructionType.JUMP_IF_GREATER_OR_EQUAL:
                a = self.stack.pop()
                b = self.stack.pop()
                if a >= b:
                    self.pc = instr.value - 1
            elif instr.type == InstructionType.RETURN:
                if self.call_stack:
                    self.pc = self.call_stack.pop()
                else:
                    return self.stack.pop()
            self.pc += 1
        return None

class Program:
    @staticmethod
    def fibonacci(n):
        return [
            Instruction(InstructionType.PUSH, n),
            Instruction(InstructionType.DUPE),
            Instruction(InstructionType.PUSH, 1),
            Instruction(InstructionType.JUMP_IF_GREATER_OR_EQUAL, 15),
            Instruction(InstructionType.DUPE),
            Instruction(InstructionType.PUSH, 1),
            Instruction(InstructionType.ROTATE),
            Instruction(InstructionType.SUBTRACT),
            Instruction(InstructionType.CALL, 1),
            Instruction(InstructionType.ROTATE),
            Instruction(InstructionType.PUSH, 2),
            Instruction(InstructionType.ROTATE),
            Instruction(InstructionType.SUBTRACT),
            Instruction(InstructionType.CALL, 1),
            Instruction(InstructionType.ADD),
            Instruction(InstructionType.RETURN)
        ]

def native_fibonacci(n):
    if n <= 1:
        return n
    return native_fibonacci(n - 1) + native_fibonacci(n - 2)

if __name__ == '__main__':
    import sys
    n = int(sys.argv[1])
    native = sys.argv[2] == '--native' if len(sys.argv) > 2 else False
    
    if native:
        print(native_fibonacci(n))
    else:
        code = Program.fibonacci(n)
        vm = VM(code)
        print(vm.run())
