
using System.Diagnostics;
using System.Threading.Channels;
using Channel = System.Threading.Channels.Channel;
//https://alexyakunin.medium.com/go-vs-c-part-1-goroutines-vs-async-await-ac909c651c11

namespace ChannelsTest
{
    class Program
    {
        public static void Measure(string title, Action<int, bool> test, int count, int warmupCount = 1)
        {
            var sw = new Stopwatch();
            sw.Start();
            test(count, false);
            sw.Stop();
            Console.WriteLine($"{title}: {sw.Elapsed.TotalMilliseconds:0.000}ms");
        }

        static async void AddOne(ChannelWriter<int> output, ChannelReader<int> input)
        {
            await output.WriteAsync(1 + await input.ReadAsync());
        }
        
        public static void test(int maxCount) 
        {
            Measure($"Sending {maxCount} messages (channels)", (count, isWarmup) => {
                var firstChannel = Channel.CreateUnbounded<int>();
                var output = firstChannel;
                for (var i = 0; i < count; i++) {
                    var input = Channel.CreateUnbounded<int>();
                    AddOne(output.Writer ,input.Reader);
                    output = input;
                }
                output.Writer.WriteAsync(0);
                
            
            }, maxCount);
        }
        static void Main(string[] args)
        {
            if (!int.TryParse(args.FirstOrDefault(), out var maxCount))
                maxCount = 100000;
            test(maxCount);
            test(maxCount);
            test(maxCount);
            test(maxCount);
        }
    }
}




