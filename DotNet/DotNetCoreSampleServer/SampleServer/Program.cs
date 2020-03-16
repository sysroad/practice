using System;
using System.Runtime.InteropServices;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.DependencyInjection;

namespace SampleServer
{
    class Program
    {
        static void Main(string[] args)
        {
            CreateHostBuilder(args, LogLevel.Debug)?.Build().Run();
        }

        public static IHostBuilder CreateHostBuilder(string[] args, LogLevel minLevel)
        {
            IHostBuilder builder = null;

            if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
            {
                builder = Host.CreateDefaultBuilder(args).UseSystemd();
            }
            else if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                builder = Host.CreateDefaultBuilder(args).UseWindowsService();
            }

            return builder?.ConfigureServices((hostcontext, services) =>
            {
                services.AddHostedService<Worker>().AddLogging((logbuilder) => 
                {
                    logbuilder.AddFilter((level) => { return minLevel <= level; });
                });
            });
        }
    }
}
