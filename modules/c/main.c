#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

typedef enum {
    Push,
    Add,
    Subtract,
    Dupe,
    Rotate,
    Call,
    JumpIfGreaterOrEqual,
    Return
} InstructionType;

typedef struct {
    InstructionType type;
    int value;
} Instruction;

typedef struct {
    int *stack;
    size_t stack_size;
    size_t stack_capacity;
    size_t *call_stack;
    size_t call_stack_size;
    size_t call_stack_capacity;
    size_t pc;
    Instruction *code;
    size_t code_size;
} VM;

VM *vm_new(Instruction *code, size_t code_size) {
    VM *vm = (VM *)malloc(sizeof(VM));
    vm->stack = (int *)malloc(16 * sizeof(int));
    vm->stack_size = 0;
    vm->stack_capacity = 16;
    vm->call_stack = (size_t *)malloc(16 * sizeof(size_t));
    vm->call_stack_size = 0;
    vm->call_stack_capacity = 16;
    vm->pc = 0;
    vm->code = code;
    vm->code_size = code_size;
    return vm;
}

void vm_push(VM *vm, int value) {
    if (vm->stack_size >= vm->stack_capacity) {
        vm->stack_capacity *= 2;
        vm->stack = (int *)realloc(vm->stack, vm->stack_capacity * sizeof(int));
    }
    vm->stack[vm->stack_size++] = value;
}

int vm_pop(VM *vm) {
    return vm->stack[--vm->stack_size];
}

int vm_last(VM *vm) {
    return vm->stack[vm->stack_size - 1];
}

void vm_push_call(VM *vm, size_t address) {
    if (vm->call_stack_size >= vm->call_stack_capacity) {
        vm->call_stack_capacity *= 2;
        vm->call_stack = (size_t *)realloc(vm->call_stack, vm->call_stack_capacity * sizeof(size_t));
    }
    vm->call_stack[vm->call_stack_size++] = address;
}

size_t vm_pop_call(VM *vm) {
    return vm->call_stack[--vm->call_stack_size];
}

int vm_run(VM *vm) {
    while (vm->pc < vm->code_size) {
        Instruction instr = vm->code[vm->pc];
        switch (instr.type) {
            case Push:
                vm_push(vm, instr.value);
                break;
            case Add: {
                int a = vm_pop(vm);
                int b = vm_pop(vm);
                vm_push(vm, a + b);
                break;
            }
            case Subtract: {
                int a = vm_pop(vm);
                int b = vm_pop(vm);
                vm_push(vm, a - b);
                break;
            }
            case Dupe:
                vm_push(vm, vm_last(vm));
                break;
            case Rotate: {
                int a = vm_pop(vm);
                int b = vm_pop(vm);
                vm_push(vm, a);
                vm_push(vm, b);
                break;
            }
            case Call:
                vm_push_call(vm, vm->pc);
                vm->pc = instr.value - 1;
                break;
            case JumpIfGreaterOrEqual: {
                int a = vm_pop(vm);
                int b = vm_pop(vm);
                if (a >= b) {
                    vm->pc = instr.value - 1;
                }
                break;
            }
            case Return:
                if (vm->call_stack_size > 0) {
                    vm->pc = vm_pop_call(vm);
                } else {
                    return vm_pop(vm);
                }
                break;
        }
        vm->pc++;
    }
    return 0;
}

Instruction *program_fibonacci(int n, size_t *size) {
    *size = 16;
    Instruction *code = (Instruction *)malloc(*size * sizeof(Instruction));
    code[0] = (Instruction){Push, n};
    code[1] = (Instruction){Dupe, 0};
    code[2] = (Instruction){Push, 1};
    code[3] = (Instruction){JumpIfGreaterOrEqual, 15};
    code[4] = (Instruction){Dupe, 0};
    code[5] = (Instruction){Push, 1};
    code[6] = (Instruction){Rotate, 0};
    code[7] = (Instruction){Subtract, 0};
    code[8] = (Instruction){Call, 1};
    code[9] = (Instruction){Rotate, 0};
    code[10] = (Instruction){Push, 2};
    code[11] = (Instruction){Rotate, 0};
    code[12] = (Instruction){Subtract, 0};
    code[13] = (Instruction){Call, 1};
    code[14] = (Instruction){Add, 0};
    code[15] = (Instruction){Return, 0};
    return code;
}

int native_fibonacci(int n) {
    if (n <= 1) {
        return n;
    }
    return native_fibonacci(n - 1) + native_fibonacci(n - 2);
}

int main(int argc, char **argv) {
    if (argc != 2 && argc != 3) {
        fprintf(stderr, "Usage: %s <n> [--native]\n", argv[0]);
        return 1;
    }

    int n = atoi(argv[1]);
    bool native = argc == 3 && strcmp(argv[2], "--native") == 0;

    if(native) {
        int result = native_fibonacci(n);
        printf("%d\n", result);
    } else {
        size_t code_size;
        Instruction *code = program_fibonacci(n, &code_size);
        VM *vm = vm_new(code, code_size);
        int result = vm_run(vm);
        printf("%d\n", result);
        free(vm->stack);
        free(vm->call_stack);
        free(vm);
        free(code);
    }

    return 0;    
}
