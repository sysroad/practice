using SuperSocket.SocketBase;
using SuperSocket.SocketBase.Protocol;
using Microsoft.Extensions.Logging;

namespace SampleServer
{
    public class SampleSession : AppSession<SampleSession, StringRequestInfo>
    {
        protected override void HandleUnknownRequest(StringRequestInfo requestInfo)
        {
            Global.Logger.LogDebug($"request key : {requestInfo.Key}");
            Global.Logger.LogDebug($"request body : {requestInfo.Body}");
        }

        protected override void HandleException(System.Exception e)
        {
            base.HandleException(e);
        }
    }
}