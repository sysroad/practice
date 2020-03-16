using SuperSocket.SocketBase;
using Microsoft.Extensions.Logging;

namespace SampleServer
{
    public class SampleServer : AppServer<SampleSession>
    {
        protected override void OnNewSessionConnected(SampleSession session)
        {
            Global.Logger.LogDebug($"{session.SessionID} connected");
        }

        protected override void OnSessionClosed(SampleSession session, CloseReason reason)
        {
            Global.Logger.LogDebug($"{session.SessionID} closed");
        }
    }
}