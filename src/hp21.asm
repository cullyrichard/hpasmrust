******************************************************************
***
***  HP 2116A/2115A Computer System
***  Shift/Rotate Demonstation Program
***
***  1967:	Original
***  2004 Apr:	This re-creation / bhilpert
***  2014 Nov:	Adapted for original hp syntax /bh
***
******************************************************************
*
* This is a recreation of a program from the back of
* the "2116A Vol. 1" manual.
*
******************************************************************

		org	00001000

Start		LDA	_ldInstrInit	INITIALIZE
		STA	_ldInstr	Get first Load instruction.

_instrLoop	CLE
_ldInstr	abs	var		LOAD shift instruction
		STB	_srInstr	 into loop.
		CPB	_lastInstr	PUT pattern into A.
		JMP	_ldPattern1	 If ALF, use 000001.
		LDA	_pattern2	 All others use 100401.
		JMP	_repInit
_ldPattern1	CLA,INA

_repInit	LDB	_Const16	INITIALIZE Shift Counter.
		CMB,INB			Set to -16.
		STB	_repCounter

_repLoop	CLB			INITIALIZE Timer.
		STB	_timerMinor	Set to loop for 1 second.
		LDB	_Const6
		CMB,INB
		STB	_timerMajor
*_timerLoop	CLB			** sim timing
*		CMB			** sim timing
*		STB	_timerMinor	** sim timing
_timerLoop	ISZ	_timerMinor	LOOP.
		JMP	_timerLoop	 One second.
		ISZ	_timerMajor
		JMP	_timerLoop
	
_srInstr	ABS	0		SHIFT. (Instruction loaded by 3003.)

		ISZ	_repCounter	LOOP.
		JMP	_repLoop	 16 shifts, one per seond.

		LDB	_lastInstr	CHECK
		CPB	_srInstr	 for last instruction.
		HLT	0		HALT.
		ISZ	_ldInstr	CHANGE instruction
		JMP	_instrLoop	 and repeat demonstration.

******************************************************************
*  Data

_firstInstr	ALS	
		ARS
		RAL
		RAR
		ALR
		ERA
		ELA
_lastInstr	ALF

_timerMinor	abs	var
_timerMajor	abs	var
_Const6		dec	6
_repCounter	abs	var
_Const16	oct	020
_pattern2	oct	0100401
_ldInstrInit	LDB	_firstInstr

var		equ	0

******************************************************************

