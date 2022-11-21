with Ada.Text_IO;
with Ada.Integer_Text_IO;
with Ada.Streams.Stream_IO; use Ada.Streams.Stream_IO;

procedure Exo1 is
    bin: constant String := "/nix/store/qarssrazji0q9xp80xg8shsm2crckfr0-coreutils-9.0/bin/cat";
    F: File_Type;
    S: Stream_Access;

    type Int7Bytes is new Integer range 0 .. 7205;
    for Int7Bytes'Size use 90;
    package Int7Bytes_IO is new Ada.Text_IO.Integer_IO(Int7Bytes);

    type Int1Bytes is new Integer range 0 .. 255;
    for Int1Bytes'Size use 8;
    package Int1Bytes_IO is new Ada.Text_IO.Integer_IO(Int1Bytes);

    type Int2Bytes is new Integer range 0 .. 65535;
    for Int2Bytes'Size use 16;
    package Int2Bytes_IO is new Ada.Text_IO.Integer_IO(Int2Bytes);

    type MagicElfRange is range 0 .. 2;
    type MagicElf is array(MagicElfRange) of Character;

    type ELF is record
        e_magic: Int1Bytes;
        e_magic2: MagicElf;
        e_class: Int1Bytes;
        e_data: Int1Bytes;
        e_version: Int1Bytes;
        e_osabit: Int1Bytes;
        e_abiversion: Int1Bytes;

        e_padding1: Int1Bytes;
        e_padding2: Int1Bytes;
        e_padding3: Int1Bytes;

        e_padding4: Int1Bytes;
        e_padding5: Int1Bytes;
        e_padding6: Int1Bytes;
        e_padding7: Int1Bytes;

        e_type: Int2Bytes;
        e_machine: Int2Bytes;

    end record;

    elf_binary: ELF;

    procedure Print_Class(elf_binary: ELF) is 
    begin
        case elf_binary.e_class is
            when 1 =>  Ada.Text_IO.Put_Line("32 Bit");
            when 2 => Ada.Text_IO.Put_Line("64 Bit");
            when others => Ada.Text_IO.Put_Line("Unknow Class");
        end case;
    end Print_Class;

    procedure Print_Data(elf_binary: ELF) is 
    begin
        case elf_binary.e_data is
            when 1 =>  Ada.Text_IO.Put_Line("Little endianness");
            when 2 => Ada.Text_IO.Put_Line("Big endianness");
            when others => Ada.Text_IO.Put_Line("Unknow Data");
        end case;
    end Print_Data;

    procedure Print_OsAbi(elf_binary: ELF) is 
    begin
        case elf_binary.e_osabit is
            when 16#0# => Ada.Text_IO.Put_Line("System V");
            when 16#1# => Ada.Text_IO.Put_Line("HP-UX");
            when 16#2# => Ada.Text_IO.Put_Line("NetBSD");
            when 16#3# => Ada.Text_IO.Put_Line("Linux");
            when 16#4# => Ada.Text_IO.Put_Line("GNU Hurd");
            when 16#6# => Ada.Text_IO.Put_Line("Solaris");
            when 16#7# => Ada.Text_IO.Put_Line("AIX (Monterey)");
            when 16#8# => Ada.Text_IO.Put_Line("IRIX");
            when 16#9# => Ada.Text_IO.Put_Line("FreeBSD");
            when 16#A# => Ada.Text_IO.Put_Line("Tru64");
            when 16#B# => Ada.Text_IO.Put_Line("Novell Modesto");
            when 16#C# => Ada.Text_IO.Put_Line("OpenBSD");
            when 16#D# => Ada.Text_IO.Put_Line("OpenVMS");
            when 16#E# => Ada.Text_IO.Put_Line("NonStop Kernel");
            when 16#F# => Ada.Text_IO.Put_Line("AROS");
            when 16#10# => Ada.Text_IO.Put_Line("FenixOS");
            when 16#11# => Ada.Text_IO.Put_Line("Nuxi CloudABI");
            when 16#12# => Ada.Text_IO.Put_Line("Stratus Technologies OpenVOS");
            when others => Int1Bytes_IO.Put(elf_binary.e_osabit);
        end case;
    end Print_OsAbi;

    procedure Print_Type(elf_binary: ELF) is 
    begin
        case elf_binary.e_type is
            when 16#00# => Ada.Text_IO.Put_Line("ET_NONE Unknown.");
            when 16#01# => Ada.Text_IO.Put_Line("ET_REL Relocatable file.");
            when 16#02# => Ada.Text_IO.Put_Line("ET_EXEC Executable file.");
            when 16#03# => Ada.Text_IO.Put_Line("ET_DYN Shared object.");
            when 16#04# => Ada.Text_IO.Put_Line("ET_CORE Core file.");
            when others => Int2Bytes_IO.Put(elf_binary.e_type);
        end case;
    end Print_Type;

    procedure Print_Machine(elf_binary: ELF) is 
    begin
        case elf_binary.e_machine is
            when 16#00# => Ada.Text_IO.Put_Line("No specific instruction set");
            when 16#01# => Ada.Text_IO.Put_Line("AT&T WE 32100");
            when 16#02# => Ada.Text_IO.Put_Line("SPARC");
            when 16#03# => Ada.Text_IO.Put_Line("x86");
            when 16#04# => Ada.Text_IO.Put_Line("Motorola 68000 (M68k)");
            when 16#05# => Ada.Text_IO.Put_Line("Motorola 88000 (M88k)");
            when 16#06# => Ada.Text_IO.Put_Line("Intel MCU");
            when 16#07# => Ada.Text_IO.Put_Line("Intel 80860");
            when 16#08# => Ada.Text_IO.Put_Line("MIPS");
            when 16#09# => Ada.Text_IO.Put_Line("IBM System/370");
            when 16#0A# => Ada.Text_IO.Put_Line("MIPS RS3000 Little-endian");
            when 16#0E# => Ada.Text_IO.Put_Line("Hewlett-Packard PA-RISC");
            when 16#13# => Ada.Text_IO.Put_Line("Intel 80960");
            when 16#14# => Ada.Text_IO.Put_Line("PowerPC");
            when 16#15# => Ada.Text_IO.Put_Line("PowerPC (64-bit)");
            when 16#16# => Ada.Text_IO.Put_Line("S390, including S390x");
            when 16#17# => Ada.Text_IO.Put_Line("IBM SPU/SPC");
            when 16#24# => Ada.Text_IO.Put_Line("NEC V800");
            when 16#25# => Ada.Text_IO.Put_Line("Fujitsu FR20");
            when 16#26# => Ada.Text_IO.Put_Line("TRW RH-32");
            when 16#27# => Ada.Text_IO.Put_Line("Motorola RCE");
            when 16#28# => Ada.Text_IO.Put_Line("Arm (up to Armv7/AArch32)");
            when 16#29# => Ada.Text_IO.Put_Line("Digital Alpha");
            when 16#2A# => Ada.Text_IO.Put_Line("SuperH");
            when 16#2B# => Ada.Text_IO.Put_Line("SPARC Version 9");
            when 16#2C# => Ada.Text_IO.Put_Line("Siemens TriCore embedded processor");
            when 16#2D# => Ada.Text_IO.Put_Line("Argonaut RISC Core");
            when 16#2E# => Ada.Text_IO.Put_Line("Hitachi H8/300");
            when 16#2F# => Ada.Text_IO.Put_Line("Hitachi H8/300H");
            when 16#30# => Ada.Text_IO.Put_Line("Hitachi H8S");
            when 16#31# => Ada.Text_IO.Put_Line("Hitachi H8/500");
            when 16#32# => Ada.Text_IO.Put_Line("IA-64");
            when 16#33# => Ada.Text_IO.Put_Line("Stanford MIPS-X");
            when 16#34# => Ada.Text_IO.Put_Line("Motorola ColdFire");
            when 16#35# => Ada.Text_IO.Put_Line("Motorola M68HC12");
            when 16#36# => Ada.Text_IO.Put_Line("Fujitsu MMA Multimedia Accelerator");
            when 16#37# => Ada.Text_IO.Put_Line("Siemens PCP");
            when 16#38# => Ada.Text_IO.Put_Line("Sony nCPU embedded RISC processor");
            when 16#39# => Ada.Text_IO.Put_Line("Denso NDR1 microprocessor");
            when 16#3A# => Ada.Text_IO.Put_Line("Motorola Star*Core processor");
            when 16#3B# => Ada.Text_IO.Put_Line("Toyota ME16 processor");
            when 16#3C# => Ada.Text_IO.Put_Line("STMicroelectronics ST100 processor");
            when 16#3D# => Ada.Text_IO.Put_Line("Advanced Logic Corp. TinyJ embedded processor family");
            when 16#3E# => Ada.Text_IO.Put_Line("AMD x86-64");
            when 16#3F# => Ada.Text_IO.Put_Line("Sony DSP Processor");
            when 16#40# => Ada.Text_IO.Put_Line("Digital Equipment Corp. PDP-10");
            when 16#41# => Ada.Text_IO.Put_Line("Digital Equipment Corp. PDP-11");
            when 16#42# => Ada.Text_IO.Put_Line("Siemens FX66 microcontroller");
            when 16#43# => Ada.Text_IO.Put_Line("STMicroelectronics ST9+ 8/16 bit microcontroller");
            when 16#44# => Ada.Text_IO.Put_Line("STMicroelectronics ST7 8-bit microcontroller");
            when 16#45# => Ada.Text_IO.Put_Line("Motorola MC68HC16 Microcontroller");
            when 16#46# => Ada.Text_IO.Put_Line("Motorola MC68HC11 Microcontroller");
            when 16#47# => Ada.Text_IO.Put_Line("Motorola MC68HC08 Microcontroller");
            when 16#48# => Ada.Text_IO.Put_Line("Motorola MC68HC05 Microcontroller");
            when 16#49# => Ada.Text_IO.Put_Line("Silicon Graphics SVx");
            when 16#4A# => Ada.Text_IO.Put_Line("STMicroelectronics ST19 8-bit microcontroller");
            when 16#4B# => Ada.Text_IO.Put_Line("Digital VAX");
            when 16#4C# => Ada.Text_IO.Put_Line("Axis Communications 32-bit embedded processor");
            when 16#4D# => Ada.Text_IO.Put_Line("Infineon Technologies 32-bit embedded processor");
            when 16#4E# => Ada.Text_IO.Put_Line("Element 14 64-bit DSP Processor");
            when 16#4F# => Ada.Text_IO.Put_Line("LSI Logic 16-bit DSP Processor");
            when 16#8C# => Ada.Text_IO.Put_Line("TMS320C6000 Family");
            when 16#AF# => Ada.Text_IO.Put_Line("MCST Elbrus e2k");
            when 16#B7# => Ada.Text_IO.Put_Line("Arm 64-bits (Armv8/AArch64)");
            when 16#DC# => Ada.Text_IO.Put_Line("Zilog Z80");
            when 16#F3# => Ada.Text_IO.Put_Line("RISC-V");
            when 16#F7# => Ada.Text_IO.Put_Line("Berkeley Packet Filter");
            when 16#101# => Ada.Text_IO.Put_Line("WDC 65C816");
            when others => Ada.Text_IO.Put_Line("Future or Unknow Architecture");
        end case;
    end Print_Machine;

    procedure Print_ELF(elf_binary: ELF) is
    begin
        Ada.Text_IO.Put_Line("ELF Printer in ada..");
        Ada.Text_IO.Put("Magic: ");
        Int1Bytes_IO.Put(elf_binary.e_magic, Width=>8, Base => 16);
        Ada.Text_IO.Put_Line("");
        Ada.Text_IO.Put("Magic 2: ");
        for i in MagicElfRange loop
            Ada.Text_IO.Put(elf_binary.e_magic2(i));
        end loop;

        Ada.Text_IO.Put_Line("");
        Ada.Text_IO.Put("Class: ");
        Print_Class(elf_binary);

        Ada.Text_IO.Put("Endianess: ");
        Print_Data(elf_binary);

        Ada.Text_IO.Put("Version: ");
        Int1Bytes_IO.Put(elf_binary.e_version, Width=>8, Base => 16);
        Ada.Text_IO.Put_Line("");

        Ada.Text_IO.Put("Abi: ");
        Print_OsAbi(elf_binary);

        Ada.Text_IO.Put("Abi Version: ");
        Int1Bytes_IO.Put(elf_binary.e_abiversion, Width=>8, Base => 16);
        Ada.Text_IO.Put_Line("");

        Ada.Text_IO.Put("Type: ");
        Print_Type(elf_binary);

        Ada.Text_IO.Put("Machine: ");
        Print_Machine(elf_binary);
        Ada.Text_IO.Put_Line("");


    end Print_ELF;
begin
    Int7Bytes_IO.Default_Width := 8;
    Int7Bytes_IO.Default_Base := 16;

    Int1Bytes_IO.Default_Width := 8;
    Int1Bytes_IO.Default_Base := 16;

    Int2Bytes_IO.Default_Width := 8;
    Int2Bytes_IO.Default_Base := 16;

    Ada.Text_IO.Put_Line("Hello World");
    Open(F, In_File, bin);
    S := Stream(F);

    ELF'Read(S, elf_binary);
    Print_ELF(elf_binary);
    Close(F);
end Exo1;
