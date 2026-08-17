#include <mach/mach_time.h>
#include <stdint.h>
#include <stdlib.h>

double intervalInCycles( uint64_t startTime, uint64_t endTime )
{
	uint64_t rawTime = endTime - startTime;
	static double conversion = 0.0;
	
	if( 0.0 == conversion )
	{
		mach_timebase_info_data_t	info;
		kern_return_t err = mach_timebase_info( &info );
		if( 0 != err )
			return 0;
		
		uint64_t freq = 0;
		size_t freqSize = sizeof( freq );
		int err2 = sysctlbyname( "hw.cpufrequency", &freq, &freqSize, NULL, 0L );
		if( 0 != err2 )
			return 0;
		
		conversion = (double) freq * (1e-9 * (double) info.numer / (double) info.denom);
	}
	
	return (double) rawTime * conversion;
}

