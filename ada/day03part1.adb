with Ada.Assertions; use Ada.Assertions;
with Ada.Strings; use Ada.Strings;
with Ada.Strings.Fixed; use Ada.Strings.Fixed;
with Ada.Text_IO; use Ada.Text_IO;
with Interfaces; use Interfaces;

procedure Day03Part1 is
   Input_Len: Natural := 0;
   Binary_Number: Unsigned_64 := 0;
   Mask: Unsigned_64 := 0;
   Gamma: Unsigned_64 := 0;
   Epsilon: Unsigned_64;
   Product: Unsigned_64;

   -- read first number from stdin, interpret as binary number, and
   --  return input length and number
   procedure First_Number(Len: out Natural; Num: out Unsigned_64) is
      Input_String: String(1..65);
   begin
      Get_Line(Standard_Input, Input_String, Len);
      Put_Line("First_Number Input_String: " & Input_String);
      Put_Line("First_Number Len: " & Len'Image);
      Num := Unsigned_64'Value("2#" & Input_String(1..Len) & "#");
   end First_Number;

   -- read next number from stdin, which must match the predetermined
   --  length, and return the number
   procedure Next_Number(Len: in Natural; Num: out Unsigned_64) is
      Tmp_Len: Natural;
   begin
      First_Number(Tmp_Len, Num);
      Assert(Tmp_Len = Len);
   end Next_Number;

   procedure Process_Input(Len: in Natural) is
      Arr: array(1..Len) of Integer := (1..Len => 0);
   begin
      loop
	 for I in 1..Len loop
	    if (Binary_Number and Shift_Left(1, I - 1)) > 0 then
	       Arr(I) := Arr(I) + 1;
	    else
	       Arr(I) := Arr(I) - 1;
	    end if;
	 end loop;

	 exit when End_Of_File(Standard_Input);

	 Next_Number(Len, Binary_Number);
      end loop;

      -- read array to calculate Gamma
      for I in 1..Len loop
	 if Arr(I) >= 0 then
	    Gamma := Gamma or Shift_Left(1, I - 1);
	 end if;
      end loop;

   end Process_Input;
begin

   if not End_Of_File(Standard_Input) then
      -- get first input binary number and width, and initialize Mask
      First_Number(Input_Len, Binary_Number);
      Assert(Input_Len > 0);
      Mask := Shift_Left(1, Input_Len) - 1;
      Put_Line("Binary_Number: "  & Binary_Number'Image);
      Put_Line("Input_Len: "  & Input_Len'Image);
      Put_Line("Mask: " & Mask'Image);

      Process_Input(Input_Len);
   end if;

   -- calculate Epsilon and (Epsilon * Gamma)
   Epsilon := (not Gamma) and Mask;
   Product := Gamma * Epsilon;

   -- done
   Put_Line("Gamma: " & Gamma'Image);
   Put_Line("Epsilon: " & Epsilon'Image);
   Put_Line("Gamma * Epsilon: " & Product'Image);
end Day03Part1;
