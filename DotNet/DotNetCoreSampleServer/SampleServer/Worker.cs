using System.Threading;
using System.Threading.Tasks;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using SuperSocket.SocketBase.Config;

namespace SampleServer
{
    public class Worker : BackgroundService
    {
        public Worker(ILogger<Worker> logger)
        {
            Global.Logger = logger;
        }

        protected override async Task ExecuteAsync(CancellationToken stopToken)
        {
            Global.Logger.LogInformation("Service Start.");

            var server = new SampleServer();
            var setup = server.Setup(
                rootConfig: new RootConfig(),
                config: new ServerConfig
                {
                    Listeners = new [] { new ListenerConfig { Ip = "Any", Port = 9090 }},
                    TextEncoding = "UTF-8"
                }
            );

            if (setup)
            {
                server.Start();

                while (!stopToken.IsCancellationRequested)
                {
                    await Task.Delay(1000, stopToken);
                }
            }

            Global.Logger.LogInformation("Service Stopped.");
        }
    }
}