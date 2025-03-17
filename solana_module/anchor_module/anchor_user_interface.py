# MIT License
#
# Copyright (c) 2025 Manuel Boi - Università degli Studi di Cagliari
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
# THE SOFTWARE.


import asyncio
from solana_module.anchor_module.automatic_data_insertion_manager import run_execution_trace
from solana_module.anchor_module.pda_key_generator import choose_program_for_pda_generation
from solana_module.anchor_module.program_compiler_and_deployer import compile_programs
from solana_module.anchor_module.interactive_data_insertion_manager import choose_program_to_run


def choose_action():
    allowed_choices = ["1", "2", "3", "0"]
    choice = None

    # Interactive menu
    while choice != "0":
        # Print options
        print("What you wanna do?")
        print("1) Compile new program(s)")
        print("2) Run program")
        print("3) Utilities")
        print("0) Back to language selection")

        # Manage choice
        choice = input()
        if choice == "1":
            compile_programs()
        elif choice == "2":
                _choose_running_mode()
        elif choice == "3":
            _choose_utility()
        elif choice == "0":
            return
        elif choice not in allowed_choices:
            print("Please insert a valid choice.")

def _choose_running_mode():
    allowed_choices = ["1", "2", "0"]
    choice = None

    # Interactive menu
    while choice != "0":
        # Print options
        print("Which mode?")
        print("1) Interactive mode")
        print("2) Automatic mode")
        print("0) Back to Anchor menu")

        # Manage choice
        choice = input()
        if choice == "1":
            choose_program_to_run()
            return
        elif choice == "2":
            asyncio.run(run_execution_trace())
            return
        elif choice == "0":
            return
        elif choice not in allowed_choices:
            print("Please insert a valid choice.")

def _choose_utility():
    allowed_choices = ["1", "0"]
    choice = None

    # Interactive menu
    while choice != "0":
        # Print options
        print("Please choose:")
        print("1) PDA key generator")
        print("0) Back to Anchor menu")

        # Manage choice
        choice = input()
        if choice == "1":
            choose_program_for_pda_generation()
        elif choice == "0":
            return
        elif choice not in allowed_choices:
            print("Please insert a valid choice.")