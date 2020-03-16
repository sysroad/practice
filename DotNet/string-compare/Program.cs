using System;

namespace string_compare
{
    class Program
    {
        static void Main(string[] _)
        {
            string s = "hello world";
            string s2 = "hello world";

            Console.WriteLine($"s : {s}");
            Console.WriteLine($"s2: {s2}");

            if (s == s2)
            {
                Console.WriteLine("s == s2 : True");
            }
            else
            {
                Console.WriteLine("s == s2 : False");
            }

            if (s.Equals(s2))
            {
                Console.WriteLine("s.Equals(s2) : True");
            }
            else
            {
                Console.WriteLine("s.Equals(s2) : False");
            }
        }
    }
}
