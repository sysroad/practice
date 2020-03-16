using System;
using System.Text;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Hosting.Systemd;
using Microsoft.Extensions.Hosting.WindowsServices;

namespace hello_world
{
    class SomeClass
    {
        public string Name { get; private set; }
        public uint Age { get; private set; }

        public SomeClass(string name, uint age) => (Name, Age) = (name, age);
    }

    class Program
    {
        static void Main(string[] _)
        {
            string s = "헬로 월드";

            var c = s.ToCharArray();

            Console.WriteLine($"'헬로 월드' char array size is:{c.Length}");

            var b = Encoding.UTF8.GetBytes(s);

            Console.WriteLine($"'헬로 월드' bytes count is : {b.Length}");

            var s2 = Encoding.UTF8.GetString(b);

            Console.WriteLine(s2);

            SomeClass some = new SomeClass("Bella", 20);


        }
    }
}
