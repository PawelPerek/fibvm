#include <iostream>
#include <vector>
#include <stack>
#include <optional>
#include <span>
#include <cstring>

enum class InstructionType {
    Push,
    Add,
    Subtract,
    Dupe,
    Rotate,
    Call,
    JumpIfGreaterOrEqual,
    Return
};

struct Instruction {
    InstructionType type;
    int value;
};

class VM {
public:
    VM(const std::vector<Instruction>& code) : code(code), pc(0) {}

    std::optional<int> run() {
        while (pc < code.size()) {
            const auto& instr = code[pc];

            switch (instr.type) {
                case InstructionType::Push:
                    stack.push(instr.value);
                    break;
                case InstructionType::Add: {
                    auto a = pop();
                    auto b = pop();
                    stack.push(a + b);
                    break;
                }
                case InstructionType::Subtract: {
                    auto a = pop();
                    auto b = pop();
                    stack.push(a - b);
                    break;
                }
                case InstructionType::Dupe:
                    stack.push(stack.top());
                    break;
                case InstructionType::Rotate: {
                    auto a = pop();
                    auto b = pop();
                    stack.push(a);
                    stack.push(b);
                    break;
                }
                case InstructionType::Call:
                    call_stack.push(pc);
                    pc = instr.value - 1;
                    break;
                case InstructionType::JumpIfGreaterOrEqual: {
                    auto a = pop();
                    auto b = pop();
                    if (a >= b) {
                        pc = instr.value - 1;
                    }
                    break;
                }
                case InstructionType::Return:
                    if (!call_stack.empty()) {
                        pc = call_stack.top();
                        call_stack.pop();
                    } else {
                        return stack.top();
                    }
                    break;
            }
            ++pc;
        }
        return std::nullopt;
    }

private:
    int pop() {
        int value = stack.top();
        stack.pop();
        return value;
    }

    std::vector<Instruction> code;
    std::stack<int> stack;
    std::stack<size_t> call_stack;
    size_t pc;
};

class Program {
public:
    static std::vector<Instruction> fibonacci(int n) {
        return {
            {InstructionType::Push, n},                    // initialize argument
            {InstructionType::Dupe, 0},                    // stack: [n, n]
            {InstructionType::Push, 1},                    // stack: [n, n, 1]
            {InstructionType::JumpIfGreaterOrEqual, 15},   // stack: [n]
            {InstructionType::Dupe, 0},                    // stack: [n, n]
            {InstructionType::Push, 1},                    // stack: [n, n, 1]
            {InstructionType::Rotate, 0},                  // stack: [n, 1, n]
            {InstructionType::Subtract, 0},                // stack: [n, n - 1]
            {InstructionType::Call, 1},                    // stack: [n, fib(n - 1)]
            {InstructionType::Rotate, 0},                  // stack: [fib(n - 1), n]
            {InstructionType::Push, 2},                    // stack: [fib(n - 1), n, 2]
            {InstructionType::Rotate, 0},                  // stack: [fib(n - 1), 2, n]
            {InstructionType::Subtract, 0},                // stack: [fib(n - 1), n - 2]
            {InstructionType::Call, 1},                    // stack: [fib(n - 1), fib(n - 2)]
            {InstructionType::Add, 0},                     // stack: [fib(n - 1) + fib(n - 2)]
            {InstructionType::Return, 0}                   // Returning value
        };
    }
};

int native_fibonacci(int n) {
    if (n <= 1) {
        return n;
    }
    return native_fibonacci(n - 1) + native_fibonacci(n - 2);
}

int main(int argc, const char* argv[]) {
    std::span args(argv, argc);

    if (args.size() != 2 && args.size() != 3) {
        fprintf(stderr, "Usage: %s <n> [--native]\n", argv[0]);
        return 1;
    }

    int n = atoi(args[1]);
    bool native = args.size() == 3 && std::strcmp(args[2], "--native") == 0;

    if(native) {
        std::cout << "Native" << std::endl;
        int result = native_fibonacci(n);
        printf("%d\n", result);
    } else {
        std::cout << "VM" << std::endl;
        auto code = Program::fibonacci(n);
        VM vm(code);
        auto result = vm.run();
        
        if (result.has_value()) {
            std::cout << result.value() << std::endl;
        } else {
            std::cout << "No result" << std::endl;
        }
    }

    return 0;
}
