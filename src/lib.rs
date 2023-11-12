mod models;

use feed_rs::parser;
use napi_derive::napi;
use std::option::Option;

#[napi]
pub fn parse(feed_string: String, feed_source: Option<String>) -> models::Feed {
  let result = parser::parse_with_uri(
    feed_string.as_bytes(),
    feed_source.as_ref().map(|source| source.as_str()),
  );

  models::Feed::from(result.unwrap())
}

#[test]
fn parse_test() {
  let rss = r#"
    <?xml version="1.0" encoding="UTF-8"?><rss xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:content="http://purl.org/rss/1.0/modules/content/" xmlns:atom="http://www.w3.org/2005/Atom" version="2.0" xmlns:media="http://search.yahoo.com/mrss/"><channel><title><![CDATA[Noop Today]]></title><description><![CDATA[Personal Developer Blog]]></description><link>https://nooptoday.com/</link><image><url>https://nooptoday.com/favicon.png</url><title>Noop Today</title><link>https://nooptoday.com/</link></image><generator>Ghost 5.37</generator><lastBuildDate>Sun, 12 Nov 2023 20:03:01 GMT</lastBuildDate><atom:link href="https://nooptoday.com/rss/" rel="self" type="application/rss+xml"/><ttl>60</ttl><item><title><![CDATA[How To Run NestJS with Bun [Simplest Guide]]]></title><description><![CDATA[You can run NestJS with Bun]]></description><link>https://nooptoday.com/how-to-run-nestjs-with-bun/</link><guid isPermaLink="false">65008aae268a8900014b5cf6</guid><category><![CDATA[bun]]></category><category><![CDATA[bunjs]]></category><category><![CDATA[nestjs]]></category><category><![CDATA[nodejs]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Tue, 12 Sep 2023 16:14:49 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/09/bun-nestjs-cover.jpg" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/09/bun-nestjs-cover.jpg" alt="How To Run NestJS with Bun [Simplest Guide]"><p>Bun released its version 1.0!</p><p>If you go to the <a href="https://bun.sh/blog/bun-v1.0?ref=noop-today">release notes</a>, the most exciting part of Bun is... </p><p><strong>It is ( almost ) &#xA0;completely compatible with ( almost ) all the existing Node.js ecosystem.</strong></p><p>Actually, they made it so compatible, some compatibility issues in Node.js doesn&apos;t even exist for Bun.</p><p>The most notable is module system, <code>import</code> and <code>require</code> keywords can exist in harmony without ever writing <code>type: module</code> to your package.json!</p><h2 id="edit-12-september-2023">Edit: 12 September 2023</h2><figure class="kg-card kg-image-card kg-card-hascaption"><img src="https://nooptoday.com/content/images/2023/09/image.png" class="kg-image" alt="How To Run NestJS with Bun [Simplest Guide]" loading="lazy" width="1782" height="1256" srcset="https://nooptoday.com/content/images/size/w600/2023/09/image.png 600w, https://nooptoday.com/content/images/size/w1000/2023/09/image.png 1000w, https://nooptoday.com/content/images/size/w1600/2023/09/image.png 1600w, https://nooptoday.com/content/images/2023/09/image.png 1782w" sizes="(min-width: 720px) 720px"><figcaption>discord talk with jmcdo29 about bun support of nestjs</figcaption></figure><p>Well guys, that happened. Appearantly, I was tricked by the Bun commercial. Because, they advertised that <a href="https://github.com/oven-sh/bun/issues/1641?ref=noop-today#issuecomment-1713680910">Bun works with NestJS</a>.</p><p>As I am excited about the Bun release, I jumped to the conclusions too quick. Appearantly, if you use <code>bun start</code> it doesn&apos;t actually use Bun under the hood. Thankfully <code>@jmcdo29</code> mentioned this issue in the <a href="https://discord.gg/nestjs?ref=noop-today">Discord group of NestJS.</a></p><p>If anyone can&apos;t load the image, here is the explanation:</p><blockquote>By the way, <code>bun start</code> on a <code>nest new</code> project calls <code>nest start</code> which uses the <code>nest</code> CLI which has a <code>node</code> shebang, so it&apos;s using <code>node</code>, not bun.<br>- jmcdo29</blockquote><p>The explanation below is not applicable currently.</p><h2 id="how-to-run-nestjs-with-bun">How to Run NestJS with Bun</h2><p>Okay, but how do you run NestJS with Bun?</p><p>Well, it is a problem of the <a href="https://www.reddit.com/r/Nestjs_framework/comments/wk2z00/anyone_managed_to_run_nestjs_on_bun_runtime/?ref=noop-today">past</a>.</p><p>Simply:</p><pre><code class="language-bash">bun start</code></pre><p>If this doesn&apos;t work, install Bun first:</p><pre><code class="language-bash">curl -fsSL https://bun.sh/install | bash</code></pre><p>Don&apos;t you trust curl commands? &#xA0;Then you can use npm to install Bun:</p><pre><code class="language-bash">npm install -g bun</code></pre><p>NestJS or any other web framework popular enough is guaranteed to work with Bun.</p>]]></content:encoded></item><item><title><![CDATA[What Would Marco Polo Think About the WebSockets?]]></title><description><![CDATA[<p>Marco Polo and WebSockets? You might be surprised to hear about their relationship.</p><p>On his visit to Asia, Marco Polo was impressed by the Yam, a postal system built by the Mongol Empire.</p><p>Still no connection? Okay, let&apos;s rewind a bit.</p><hr><p>In this fast-paced world, we want everything</p>]]></description><link>https://nooptoday.com/what-would-marco-polo-think-about-the-websockets/</link><guid isPermaLink="false">64e0867b268a8900014b5b7e</guid><category><![CDATA[websockets]]></category><category><![CDATA[network]]></category><category><![CDATA[protocol]]></category><category><![CDATA[backend]]></category><category><![CDATA[ack]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Sat, 19 Aug 2023 11:55:40 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/08/mongolian_cover.jpg" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/08/mongolian_cover.jpg" alt="What Would Marco Polo Think About the WebSockets?"><p>Marco Polo and WebSockets? You might be surprised to hear about their relationship.</p><p>On his visit to Asia, Marco Polo was impressed by the Yam, a postal system built by the Mongol Empire.</p><p>Still no connection? Okay, let&apos;s rewind a bit.</p><hr><p>In this fast-paced world, we want everything to happen in real time.</p><p>We want real-time communication, real-time updates on exchanges, real-time money transfers, real-time teleportation, real-time this, and real-time that!</p><p>In the old times, real-time communication wasn&apos;t possible. Moreover, communicating over long distances was a lengthy process.</p><p>But it was crucial for the Khans of the Mongol Empire, as it was a geographically large empire. So, they built one of the best postal systems at the time, called the <a href="https://en.wikipedia.org/wiki/Yam_(route)?ref=noop-today">Yam</a>.</p><p>In my research of the Yam, I couldn&apos;t help but notice how similar it is to the Internet we use today.</p><p>In this post, I want to provide a different perspective. A perspective that shines a light on the evolution of real-time technologies of today. Have a great read!</p><h2 id="letters">Letters</h2><p>One of the oldest and most complete ways of communication is sending letters. Previous methods of communication lacked some essential features. So what did the letters provide? Why they were so powerful and popular throughout history?</p><p>If you remember from school, there are five fundamental elements to communication:</p><ul><li>The source ( sender )</li><li>The message</li><li>The channel</li><li>The target ( receiver )</li><li>Feedback ( Essential for two-ways communication )</li></ul><p>So, if I send a letter to my brother:</p><ul><li>I am the source,</li><li>the message is the text in the letter,</li><li>the letter is the channel,</li><li>my brother is the target.</li></ul><p>It is strange that even though we use very different technologies to communicate, the fundamentals are the same. We can find WebSocket equivalents of all the things above. In fact, we can find TCP, HTTP, SMTP, or any other protocol equivalent to these. But let&apos;s keep it in the WebSockets.</p><p>WebSocket protocol is built on top of TCP. In other words, the clients must establish a TCP connection to the server first. Then, they can switch to the WebSocket protocol via an upgrade mechanism.</p><p>So, a WebSocket equivalent to the above example would be as follows:</p><ul><li>Sender IP and Sender Port is the source,</li><li>Bytes of data in the TCP packet are the message,</li><li>Internet / Web is the channel,</li><li>The receiver IP and the Receiver Port is the target.</li></ul><p>What about the feedback? Well, if he doesn&apos;t write me back there is no feedback and the communication is one-sided. Therefore, a letter contains the address of the receiver as well as the address of the sender, so the receiver can reply.</p><p>In WebSockets, there is no concept of feedback. Even though the underlying TCP connection ensures message delivery via ACK packets, the WebSocket protocol doesn&apos;t have such a concept. For that reason, some other protocols exist, such as Socket.IO, Stomp, and more. Those protocols offer a mechanism to ACKnowledge ( Feedback ) that the receiver received the message.</p><p>Apart from fundamental elements, there are other very important aspects to communication.</p><h3 id="proving-the-authenticity-of-a-letter">Proving the Authenticity of A Letter</h3><p>The receiver wants to ensure the message is authentic. Is it coming from the source? There were several mechanisms to ensure the authenticity of a letter.</p><p>To start with, people used stamps and their signatures to prove the letter is written by them. Assuming the receiver knows who uses which stamp and how their signatures look, they can verify the letter is indeed authentic.</p><p>This immediately reminds me of today&apos;s Internet. Browsers check the authenticity of the websites by looking at their signatures. In other words, SSL certificates. For example, you can check that you are at <a href="https://nooptoday.com/">https://nooptoday.com</a> and verify that this is indeed from the correct source!</p><p>Secondly, people could recognize the handwriting of the sender. They could recognize the writing style of the sender. Humans are actually very capable of recognizing patterns. During the days of World War 2, <a href="https://axbom.com/keystroke-dynamics/?ref=noop-today">telegraph operators were able to distinguish senders by their tapping rhythm and style.</a> </p><p>This is somewhat persistent throughout all communication methods. For example, if you are a long-time Noop Today reader, you might realize my recent post about RSS feeds was not written by me. It was written by AI. But, if you are a newcomer, you might not realize it.</p><p>People also sent encrypted messages to each other. This further helps prove the authenticity of the sender, assuming only the sender knows how to encrypt the message.</p><p>This is again accomplished by using SSL certificates on the websites. Most web protocols also have their secure counterparts: HTTP &amp; HTTPS, WS &amp; WSS, or SMTP &amp; SMTPS.</p><p>These methods were not great but they worked to some extent. Of course, as you may have noticed, all these methods require a pre-established trust between the parties.</p><blockquote>Even today, the Internet contains many security flaws. At least, they are fixed as soon as they are detected. Still, it doesn&apos;t mean every web service on the Internet complies with the latest bestest security practices.</blockquote><h3 id="more-about-letters">More About Letters</h3><p>The letter is a physical channel for communication. This allowed senders to add fragrance to their letters, use quality papers to reflect the importance of their message, or decorate the paper in beautiful ways to convey more than words in their message.</p><p>However, it also means that letters can be lost during transmission, they can be stolen or damaged along the way. This brings us to another important aspect of communication: Reliability.</p><h3 id="delivering-messages-in-a-reliable-way">Delivering Messages in A Reliable Way</h3><p>Well, you&apos;ve prepared a beautiful letter, sprayed it with a nice perfume, and used the best quality paper for it. You hit send, ehm. excuse me, you gave your letter to a messenger, and you are not sure whether it will reach the receiver.</p><p>What if your message is really important?</p><p>Then you could give your message to a very reputable courier. Which is very similar to using a more trusted service for sending messages from our phones.</p><p>If you don&apos;t hear back from the receiver, you can send it again. This method is used by the TCP protocol. The receiver might receive more than one message, but since you send the letter again and again, it will be delivered at least once. This is the algorithm behind <a href="https://www.rabbitmq.com/reliability.html?ref=noop-today#confirms">at least once delivery</a> guarantee provided by message broker softwares such as RabbitMQ, Kafka or Redis.</p><p>People used <a href="https://www.quora.com/How-were-letters-sent-and-delivered-during-the-Renaissance?ref=noop-today">very different methods to send their letters</a> until generally available postal offices were built.</p><h2 id="the-need-for-speed-in-information-exchange">The Need for Speed in Information Exchange</h2><p>The importance of the communication speed is undisputable. But how important it was? I think we can judge its importance for the Mongol Empire, just by looking at how much resources and effort was put into building the Yam.</p><p>It consisted of numerous postal stations ( 1400 in China alone ), countless horses ( up to 50,000 ), and a large number of oxen, mules, carts, boats, dogs, and sheep.</p><p>The goal was to speed up the information exchange within the country. The Mongol Empire was geographically large, and it was crucial to deliver information in a short time from one end of the country to the other.</p><p>China had a similar system before the Yam, but the Mongols took it too far to build a complete information network.</p><h3 id="how-did-the-yam-postal-system-worked">How did The Yam Postal System Worked?</h3><p>The Yam, focused heavily on speed. The postal stations acted as relays. Every postal station contained some amount of horses, and messengers ready to go in demand. Moreover, postal stations are located about 20-40 miles from each other.</p><p>When a message is sent from a postal station, the messenger immediately starts riding to the nearest postal station along the route.</p><p>The initial messenger delivers the letter to the next postal station. Another messenger from the station immediately starts riding to the next station and so on until the letter is reached to the final destination.</p><p>This process is called relaying. The message is relayed through intermediate postal stations.</p><p>Now, I want you to make an experiment. Open your terminal and enter the following command:</p><pre><code class="language-bash"># On Macos and Linux
traceroute nooptoday.com

# On Windows:
tracert nooptoday.com</code></pre><p>Wait a moment until the command completes, and you will see a similar output:</p><pre><code class="language-bash">traceroute: Warning: nooptoday.com has multiple addresses; using 188.114.97.3
traceroute to nooptoday.com (188.114.97.3), 64 hops max, 52 byte packets
 1  192.168.1.1 (192.168.1.1)  2.569 ms  2.190 ms  2.045 ms
 2  * * *
 3  172.19.6.129 (172.19.6.129)  31.203 ms  37.910 ms  29.936 ms
 4  172.16.193.125 (172.16.193.125)  39.917 ms  42.795 ms  35.853 ms
 5  172.16.60.86 (172.16.60.86)  24.988 ms
    172.16.60.134 (172.16.60.134)  33.023 ms  39.132 ms
 6  172.16.60.101 (172.16.60.101)  31.876 ms
    172.16.60.149 (172.16.60.149)  40.284 ms  42.080 ms
 7  172.16.198.3 (172.16.198.3)  36.744 ms
    172.16.198.7 (172.16.198.7)  58.546 ms
    172.16.198.3 (172.16.198.3)  51.061 ms
 8  10.40.168.240 (10.40.168.240)  31.211 ms
    10.40.141.65 (10.40.141.65)  45.503 ms
    10.40.169.122 (10.40.169.122)  60.582 ms
 9  ae6.bucarest1.buc.seabone.net (93.186.132.6)  89.756 ms  77.439 ms  87.002 ms
10  cloudflare.bucarest1.buc.seabone.net (93.186.132.11)  80.037 ms  107.892 ms  76.112 ms
11  188.114.97.3 (188.114.97.3)  80.520 ms  76.317 ms  86.985 ms</code></pre><p>Traceroute allows you to see exactly which hops your network packets go through until reaching to the destination server. The hops are the servers between you and the destination, a.k.a relays. Depending on your geographical location the results will be different for you.</p><p>I am using Cloudflare to secure my server, so termination address should belong to the cloudflare as in my example.</p><p>What? Are you not impressed? Okay, how about some visuals? This is the visualization of my connection from Turkey, to the twitter.com. You can try and see the results for yourselves at <a href="https://stefansundin.github.io/traceroute-mapper/?ref=noop-today">https://stefansundin.github.io/traceroute-mapper/</a></p><figure class="kg-card kg-image-card"><img src="https://nooptoday.com/content/images/2023/08/image-1.png" class="kg-image" alt="What Would Marco Polo Think About the WebSockets?" loading="lazy" width="2000" height="848" srcset="https://nooptoday.com/content/images/size/w600/2023/08/image-1.png 600w, https://nooptoday.com/content/images/size/w1000/2023/08/image-1.png 1000w, https://nooptoday.com/content/images/size/w1600/2023/08/image-1.png 1600w, https://nooptoday.com/content/images/2023/08/image-1.png 2274w" sizes="(min-width: 720px) 720px"></figure><p>Okay, so relaying is pretty much the same compared to the Yam.</p><p>A minor difference is that, messengers in the Yam system cared about the geographical distance and the actual geographical route to their destination, and decided which station they should go next.</p><p>The <a href="https://www.cloudflare.com/learning/network-layer/internet-protocol/?ref=noop-today">network packets are routed according to the IP protocol</a>, and it works a little different. Whatever the case is, the main purpose of the IP protocol is to deliver the packages to the destination and do it efficiently.</p><p>The Yam was optimized to prevent any delays during the transmissions. They contained a lot of horses and riders to send out many messages without waiting for the riders to arrive.</p><p>A similar pattern in the internet protocol is the concept of Ports. You can connect to and from a computer using various ports, up to 65535 in most machines. When you open up countless chrome tabs on your computer, you are connected to each server through a randomly assigned local port of your machine. Again, don&apos;t trust me, lets have an experiment.</p><p>Open up the terminal, and enter the following:</p><pre><code class="language-bash">netstat</code></pre><p>You will see a table containing Local Address and the Foreign Address.</p><pre><code class="language-bash">Proto Recv-Q Send-Q  Local Address          Foreign Address        (state)
tcp4       0      0  192.168.1.110.51074    104.17.50.74.https     ESTABLISHED
tcp4       0      0  192.168.1.110.51073    ec2-50-17-76-205.https ESTABLISHED
tcp4       0      0  192.168.1.110.51072    server-3-160-246.https ESTABLISHED
tcp4       0      0  192.168.1.110.51071    server-3-160-246.https ESTABLISHED
tcp4       0      0  192.168.1.110.51070    server-3-160-246.https ESTABLISHED
tcp4       0      0  192.168.1.110.51069    ec2-54-88-211-15.https ESTABLISHED
tcp4       0      0  192.168.1.110.51068    ec2-54-88-211-15.https ESTABLISHED</code></pre><p>Notice that foreign addresses are usually denoted with https, which means it uses the port 443, while the local addresses contain some random numbers around 50000 range.</p><h2 id="marco-polos-thoughts-about-the-websockets-and-the-yam">Marco Polo&apos;s Thoughts About The WebSockets and The Yam</h2><p>Finally, I&apos;ve received a letter from my dear friend ChatGPT, ehm. sorry, Marco Polo.</p><blockquote class="kg-blockquote-alt">My Dearest Friend Noop Today,<br><br>I trust this missive finds you in good health and high spirits. As I recount my awe-inspiring journey through the lands of the Mongol Empire, I am compelled to share a revelation that has captured my imagination in a most profound manner. It appears that the remarkable postal system devised by the illustrious Khans, known as the &quot;Yam,&quot; bears striking resemblances not only to the intricate network of computers that weave the tapestry of our modern world, the Internet, but also to a technological marvel of our own time - the WebSocket protocol.<br><br>Imagine, if you will, the vast expanse of the Mongol Empire stretching like an endless steppe. As I traversed these lands, I marveled at the brilliance of the Yam, which connected distant corners of the empire like the sinews of a mighty horse. Dispatch riders, skilled and swift, carried messages along these well-organized routes, spanning empires and cultures. In many ways, these couriers resembled the packets of data that traverse the digital highways of our time.<br><br>But now, let us turn our gaze toward the remarkable WebSocket protocol that graces our modern age. Just as the Yam facilitated the exchange of information across great distances, so does the WebSocket protocol allow for a continuous, bidirectional flow of data between a client and a server. Like the stations of the Yam, our digital servers and data centers act as the lodges for information, housing and transmitting it seamlessly, creating a virtual channel akin to the way the Yam relay stations operated.<br><br>Moreover, the Yam and the WebSocket protocol both share the essence of real-time communication. The Yam ensured that news and orders reached their destinations promptly, just as the WebSocket protocol empowers web applications to communicate instantaneously, enriching user experiences and enabling interactive applications that mirror the swift delivery of messages along the Yam&apos;s pathways.<br><br>Additionally, both the Yam and the WebSocket protocol necessitate a certain level of centralization to function efficiently. Just as the Khans maintained control over the Yam, ensuring its proper functioning, the WebSocket protocol relies on servers to manage the connections and facilitate the data exchange, bringing a touch of governance to the flowing information streams.<br><br>The most captivating similarity, however, lies in the way these systems foster interconnectedness. The Yam linked cities and cultures, facilitating cultural exchange and commerce. Similarly, the WebSocket protocol has ushered in an era of dynamic web applications that enable real-time collaboration and engagement, connecting people across the digital landscape much like the Yam connected settlements across the vast Mongol expanse.<br><br>As I reflect upon these similarities, I am struck by the timelessness of human innovation. While the tools and technologies have evolved, the underlying desire to bridge distances, unite people, and share knowledge endures. The echoes of the Yam can be heard in the electronic pulses of the Internet and the WebSocket protocol, reminding us that human ingenuity, whether in the form of hoofbeats, digital signals, or continuous data streams, knows no bounds.<br>In closing, my friend, I implore you to contemplate the grand tapestry of history and technology that connects us all. From the desolate expanses of the Mongol Empire to the bustling networks of the Internet and the seamless interactivity of the WebSocket protocol, we are bound by a thread of innovation that stretches across time and space. May we continue to nurture this spirit of connection and progress for generations to come.<br><br>Yours in wonderment, Marco Polo</blockquote><p>It is both scary and exciting to hear Marco Polo&apos;s thoughts about the real-time technologies of today. What excites me a lot is that he keeps mentioning human progress and our desire to interconnect with each other.</p><p>I believe we&apos;ve achieved a lot since those days. It is amazing how we are all connected to each other through the internet, <em>well.. except for North Korea of course.</em></p><h2 id="summary">Summary</h2><p>I think understanding the basic concepts behind the technologies we use today is important. As a civilization, we&apos;ve achieved great advancements in technology.</p><p>However, humans&apos; basic needs and desires are still pretty much the same. In the old times, people did their best with what they had, but they didn&apos;t have today&apos;s technological tools.</p><p>I&apos;ve found it helps me to wrap my head around the various technologies of today if I know how they evolved throughout time, which needs they fulfilled, and how they were implemented before the internet era.</p><p>I hope you learned something new from this post, if you like my content please subscribe to the newsletter or leave a comment below.</p>]]></content:encoded></item><item><title><![CDATA[WebSockets and Socket.IO - Compared Side By Side]]></title><description><![CDATA[Are you a web developer looking to stay up to date on the latest technologies? Check out our blog post to learn more about the differences between WebSockets and Socket.IO!]]></description><link>https://nooptoday.com/websockets-and-socket-io-compared-side-by-side/</link><guid isPermaLink="false">64d7a686268a8900014b5b41</guid><category><![CDATA[socketio]]></category><category><![CDATA[websocket]]></category><category><![CDATA[real-time]]></category><category><![CDATA[communication]]></category><category><![CDATA[protocol]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Sat, 12 Aug 2023 15:40:14 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/08/cover-websocket-socketio.png" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/08/cover-websocket-socketio.png" alt="WebSockets and Socket.IO - Compared Side By Side"><p>In the ever-evolving world of web technology, staying updated with the latest tools and technologies is vital. Understanding the nuances of different web communication technologies can significantly influence the quality and efficiency of your web applications. Today, we will delve into the depths of two prominent web communication technologies, WebSockets and Socket.IO</p><h2 id="understanding-the-basics">Understanding the Basics</h2><p>When it comes to web development, it is crucial to have a solid understanding of the basics. One important concept to grasp is WebSockets. But what exactly are WebSockets?</p><h3 id="what-are-websockets">What are WebSockets?</h3><p>The WebSockets protocol, standardized by the Internet Engineering Task Force (IETF), facilitates full-duplex communication between clients and servers over a long-held single connection. This real-time communication protocol enables a continuous exchange of data without the need for repeated requests.</p><p>Imagine a scenario where you are using a chat application. In traditional web development, you must constantly refresh the page to check for new messages. However, with WebSockets, this is no longer necessary. A connection through WebSockets allows both the client and server to send data at any instant. This opens the door for immediate updates and swift user interactions.</p><p>WebSockets is genuinely a remarkable step towards swift, reliable, and interactive web applications. It has revolutionized the way developers approach real-time communication, making it easier than ever to create dynamic and responsive web experiences.</p><h3 id="what-is-socketio">What is Socket.IO?</h3><p>Now that we have a better understanding of WebSockets, let&apos;s dive into Socket.IO. Socket.IO is a JavaScript library that enables real-time, bidirectional, and event-based communication between clients and servers. But what sets Socket.IO apart from other libraries?</p><p>Socket.IO is not limited to WebSockets alone. It seamlessly transitions to other technologies and methods when necessary for better compatibility. This means that if a browser doesn&apos;t support WebSockets, Socket.IO will automatically use alternative methods, such as long-polling or server-sent events, to establish a connection.</p><p>However, the charm of Socket.IO lies in its capacity for emitting distinct events with arbitrary arguments. This means that you can define custom events and pass any data you want along with them. For example, in a chat application, you can emit an event called &quot;newMessage&quot; and include the message content, sender information, and timestamp as arguments.</p><p>But that&apos;s not all. Socket.IO comes packed with a bundle of features that make real-time communication even more powerful. One such feature is the ability to broadcast messages to multiple sockets. This means that when a client sends a message, it can be instantly received by all other connected clients, creating a truly collaborative environment.</p><p>Additionally, Socket.IO allows you to store data associated with each client. This can be incredibly useful when building applications that require user authentication or personalized experiences. You can easily store user-specific data and retrieve it whenever needed.</p><p>Socket.IO has become an essential tool for developers looking to create real-time applications. Its flexibility, ease of use, and extensive feature set make it a top choice for building interactive and engaging web experiences.</p><h2 id="the-inner-workings-of-websockets-and-socketio">The Inner Workings of WebSockets and Socket.IO</h2><p>WebSockets and Socket.IO are both crucial components in modern web development, enabling real-time communication between clients and servers. Understanding how these technologies work is essential for building efficient and reliable web applications.</p><h3 id="how-websockets-work">How WebSockets Work</h3><p>WebSockets start with an HTTP handshake, where the client sends a request to the server to initiate the connection. A unique element to note is the &quot;Upgrade&quot; header, which elevates the HTTP protocol to the WebSockets protocol. This transition significantly improves the speed and efficiency of data transfer.</p><p>Upon a successful handshake, an open WebSocket connection is established between the client and server. Unlike traditional HTTP connections, WebSockets allow for full-duplex communication, meaning both the client and server can send and receive data simultaneously. This continuous exchange of messages enables real-time updates and eliminates the need for frequent HTTP requests.</p><p>WebSockets use a simple message-based protocol, where data is sent in small packets called frames. These frames can carry various types of data, including text, binary, or control messages. The WebSocket API provides methods for sending and receiving these frames, making it easy for developers to work with real-time data.</p><h3 id="how-socketio-works">How Socket.IO Works</h3><p>Socket.IO, despite being ubiquitously correlated with WebSockets, isn&apos;t purely built upon them. It is a JavaScript library that abstracts the underlying transport methods, allowing developers to work with a unified and consistent API.</p><p>Socket.IO uses diverse transport methods, with WebSockets being the primary choice. It attempts the WebSocket connection initially, leveraging the advantages of real-time, bidirectional communication. However, in cases where WebSockets are not supported or cannot be established, Socket.IO gracefully falls back to alternative transport methods.</p><p>One such fallback method is HTTP long polling, where the client sends a request to the server and keeps the connection open until the server has new data to send. This technique allows for near real-time updates, even on browsers or networks that do not support WebSockets. Socket.IO also supports other fallback options like server-sent events, polling, and JSONP, depending on the client&apos;s capabilities.</p><p>These additional connectivity options dramatically increase the compatibility and reach of Socket.IO over WebSockets, resulting in a reliable yet flexible real-time communication solution. Developers can build applications that seamlessly adapt to various environments, ensuring a consistent user experience across different browsers and devices.</p><p>In conclusion, WebSockets and Socket.IO are powerful tools for enabling real-time communication in web applications. Understanding their inner workings and the advantages they offer can greatly enhance the development process and deliver exceptional user experiences.</p><h2 id="key-differences-between-websockets-and-socketio">Key Differences Between WebSockets and Socket.IO</h2><p>When it comes to real-time communication on the web, WebSockets and Socket.IO are two popular choices. While both offer similar functionality, some key differences set them apart.</p><h3 id="connection-handling">Connection Handling</h3><p>One of the main differences between WebSockets and Socket.IO lies in how they handle connections. WebSockets require manual handling of connections and reconnections. This means that developers have to write code to establish and maintain the connection between the client and the server. On the other hand, Socket.IO automates this process, providing an effortless implementation of real-time communication. It takes care of connection management, allowing developers to focus on building the application logic without worrying about the underlying connection details.</p><p>With Socket.IO, developers can easily establish a connection with the server and handle reconnections seamlessly. This makes it a preferred choice for developers who want to quickly build real-time applications without dealing with the complexities of connection management.</p><h3 id="data-transmission">Data Transmission</h3><p>Another significant difference between WebSockets and Socket.IO lies in how they transmit data. WebSockets send and receive data in a string or binary form. This means that developers have to handle the serialization and deserialization of data manually. On the other hand, Socket.IO offers event-based communication. Instead of sending raw data, Socket.IO uses distinct event names to transmit and receive data. This approach ensures the distinctive identification of data types and refines data transmission.</p><p>With Socket.IO, developers can define custom events and attach data to them. This makes it easier to organize and manage data in real-time applications. By using events, developers can create a more structured and organized communication protocol between the client and the server.</p><h3 id="compatibility-and-support">Compatibility and Support</h3><p>When it comes to compatibility and support, WebSockets and Socket.IO have their strengths and limitations. WebSockets boast wide browser support, making it a reliable choice for most modern web applications. However, WebSockets can sometimes struggle with network proxies and firewalls. These network components may interfere with WebSocket connections, causing communication issues.</p><p>Socket.IO, on the other hand, surpasses these challenges by cleverly reverting to mechanisms like long polling when WebSocket connections are unavailable. Long polling is a technique that allows the server to hold the request open until new data is available, mimicking real-time communication. This fallback mechanism ensures that Socket.IO can work in environments where WebSockets may face restrictions or limitations.</p><p>Socket.IO&apos;s ability to gracefully adapt to different network conditions makes it a more flexible choice for real-time applications that need to work in various environments.</p><p>In conclusion, while both WebSockets and Socket.IO offer real-time communication capabilities, they differ in how they handle connections, transmit data, and handle compatibility. The choice between the two depends on the specific requirements of the application and the developer&apos;s preference in terms of ease of implementation and flexibility.</p><h2 id="pros-and-cons-of-using-websockets">Pros and Cons of Using WebSockets</h2><h3 id="advantages-of-websockets">Advantages of WebSockets</h3><p>WebSockets offer several advantages over traditional HTTP/HTTPS communication. One of the key advantages is the direct communication between the client and the server. Unlike HTTP, which follows a request-response model, WebSockets allow for real-time, bidirectional communication. This means that both the client and the server can send data to each other at any time, without the need for a new request to be made.</p><p>Another advantage of WebSockets is their efficient full-duplex communication. In other words, both the client and the server can send and receive data simultaneously. This is particularly useful for applications that require constant and rapid data updates, such as chat applications or real-time collaboration tools.</p><p>Furthermore, WebSockets offer reduced latency compared to HTTP/HTTPS. In the traditional HTTP model, a new connection needs to be established for each request, resulting in increased latency. With WebSockets, the connection is kept open, allowing for instant data transfer and minimizing the delay between the client&apos;s request and the server&apos;s response.</p><h3 id="disadvantages-of-websockets">Disadvantages of WebSockets</h3><p>While WebSockets provide numerous benefits, they also come with a few disadvantages that need to be considered. One of the drawbacks is the lack of support for binary data in earlier versions of WebSockets. This means that if you need to transmit binary data, such as images or audio, you might need to use additional techniques to convert the data into a format that can be transferred using WebSockets.</p><p>In addition, WebSockets can be incompatible with some proxies and network configurations. Certain proxies and firewalls may not fully support WebSockets or may have restrictions that prevent their proper functioning. This can result in connection issues and data transfer problems for users behind such proxies or in specific network environments.</p><p>Another disadvantage of WebSockets is the need to handle reconnections manually. Unlike HTTP, which automatically handles reconnections if the connection is lost, WebSockets require explicit handling of reconnection logic. This means that developers need to implement their mechanisms to detect and recover from connection failures, ensuring a seamless user experience even in the face of network disruptions.</p><p>Despite these disadvantages, WebSockets remain a powerful tool for real-time web applications, offering efficient and low-latency communication between clients and servers. By carefully considering the pros and cons, developers can make informed decisions about when and how to utilize WebSockets in their projects.</p><h2 id="pros-and-cons-of-using-socketio">Pros and Cons of Using Socket.IO</h2><p>Socket.IO is a popular JavaScript library that enables real-time, bidirectional communication between clients and servers. It provides a range of features and functionalities that make it a powerful tool for building interactive web applications. In this section, we will explore the advantages and disadvantages of using Socket.IO</p><h3 id="advantages-of-socketio">Advantages of Socket.IO</h3><p>One of the key advantages of Socket.IO is its built-in auto-reconnection and session-handling capabilities. This means that even if a client loses its connection to the server, Socket.IO will automatically attempt to reconnect and restore the session. This feature is particularly useful in scenarios where network connectivity is unreliable or prone to interruptions.</p><p>Another advantage of Socket.IO is its built-in acknowledgment mechanism. This allows the server to receive acknowledgments from the client, ensuring that messages are successfully delivered. This feature is especially valuable in applications that require reliable message delivery, such as chat applications or real-time collaborative tools.</p><p>Socket.IO also supports broadcasting and multicasting, which allows the server to send messages to multiple clients simultaneously. This makes it easy to implement features like real-time updates or live notifications, where information needs to be pushed to multiple clients at once. The ability to efficiently distribute data across multiple clients makes Socket.IO a powerful tool for building scalable and responsive applications.</p><h3 id="disadvantages-of-socketio">Disadvantages of Socket.IO</h3><p>Despite its many advantages, Socket.IO does have a few disadvantages that developers should consider. One potential drawback is that Socket.IO can be considered overkill for simple applications that do not require real-time communication. If your application only needs to send occasional updates or does not require bidirectional communication, using Socket.IO might introduce unnecessary complexity.</p><p>Another disadvantage of Socket.IO is that it is relatively heavyweight compared to native WebSockets. Socket.IO adds an additional layer of abstraction on top of WebSockets, which can result in increased overhead and slower performance. In situations where performance is a critical factor, using native WebSockets might be a more suitable choice.</p><p>Despite these disadvantages, Socket.IO remains a popular choice for developers who require real-time communication in their applications. Its extensive feature set, ease of use, and wide adoption make it a powerful tool for building interactive and responsive web applications.</p><h2 id="practical-applications-and-use-cases">Practical Applications and Use Cases</h2><h3 id="when-to-use-websockets">When to Use WebSockets</h3><p>WebSockets shine in applications that require minimum latency and real-time data exchange, such as multiplayer games, collaborative platforms, and live-tracking applications. They are generally used when you have control over both the client and server environments.</p><p>For example, imagine you are playing an online multiplayer game with your friends. The game requires instant communication between all players to provide a seamless gaming experience. WebSockets allow the game server to send real-time updates to each player&apos;s client, ensuring that everyone is always aware of the latest game state. This enables smooth gameplay and enhances the overall gaming experience.</p><p>In another scenario, consider a collaborative platform where multiple users can work together on shared documents. With WebSockets, changes made by one user can be instantly reflected on the screens of all other users. This real-time synchronization allows for efficient collaboration and eliminates the need for constant manual refreshing or saving of documents.</p><h3 id="when-to-use-socketio">When to Use Socket.IO</h3><p>Socket.IO excels in applications where high-level, event-based real-time communication is needed. Examples include instant messaging applications, real-time analytics, and document collaboration tools. It is preferred when you need to handle varying connection capabilities and manage network complexities.</p><p>Let&apos;s say you are using a real-time analytics dashboard to monitor the performance of your website. With Socket.IO, you can receive live updates on various metrics, such as the number of active users, page load times, and conversion rates. This allows you to make data-driven decisions and quickly respond to any issues that may arise.</p><p>Furthermore, imagine you are working on a document collaboration tool where multiple users can edit a document simultaneously. Socket.IO enables real-time event-based communication between users, allowing them to see each other&apos;s changes in real time. This ensures a smooth collaborative experience and prevents conflicts when multiple users try to edit the same section of a document simultaneously.</p><h2 id="conclusion-which-one-to-choose">Conclusion: Which One to Choose?</h2><h3 id="factors-to-consider">Factors to Consider</h3><p>When choosing between WebSockets and Socket.IO, several factors come into play. This includes your project requirements, the complexity of your use case, the development resources at hand, and the expertise of your development team.</p><h3 id="final-thoughts">Final Thoughts</h3><p>WebSockets and Socket.IO are both pivotal tools in the development of real-time web applications, each with its unique strengths and trade-offs. It is essential to understand that your decision should align with your project needs and technical constraints. May your choice pave the way for an efficient, interactive, and dynamic web application!</p>]]></content:encoded></item><item><title><![CDATA[You Must Start Using RSS Now! [Detailed Guide]]]></title><description><![CDATA[Are you missing out on the latest news, updates, and articles? RSS (Really Simple Syndication) is the answer! RSS allows you to subscribe to websites and receive updates without having to visit each site individually. Learn more about how to use RSS to your advantage on our blog.]]></description><link>https://nooptoday.com/you-must-start-using-rss-now/</link><guid isPermaLink="false">64d38844268a8900014b5b1e</guid><category><![CDATA[rss]]></category><category><![CDATA[feed]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Wed, 09 Aug 2023 12:40:05 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/08/rss_cover.png" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/08/rss_cover.png" alt="You Must Start Using RSS Now! [Detailed Guide]"><p>Are you tired of missing out on the latest news, updates, and articles? Do you find yourself overwhelmed by the constant flood of information on the internet? If so, then it&apos;s time to embrace the power of RSS (Really Simple Syndication)! In this detailed guide, we will explore what RSS is, why it exists, and how you can leverage its power to stay informed and organized.## What is RSS and Why It Exists</p><p>RSS, which stands for Really Simple Syndication, is a technology that revolutionized the way people consume online content. It allows you to subscribe to websites and receive updates without having to visit each site individually. Think of it as your personal news aggregator, collecting articles, blog posts, and podcasts from your favorite sources and delivering them directly to you.</p><p>The purpose of RSS is to simplify the process of consuming online content. In today&apos;s fast-paced digital world, where information is constantly being generated and shared, it can be overwhelming to keep up with all the websites and blogs that you follow. RSS brings everything you care about into one place, making it easier for you to stay informed and up-to-date.</p><p>Imagine this: instead of spending time visiting multiple websites and sifting through countless articles, RSS does the work for you. It gathers the latest content from your preferred sources and presents it in a clean and organized format. Whether you&apos;re interested in news, technology, sports, or any other topic, RSS allows you to customize your feed and receive only the content that matters to you.</p><p>But why does RSS exist in the first place? The answer lies in the ever-evolving nature of the internet. As the number of websites and blogs grew exponentially over the years, it became increasingly difficult for users to keep track of their favorite sources. People needed a way to efficiently consume content without wasting time and effort.</p><p>RSS was created to address this need. It provides a standardized format for websites to publish their content in a machine-readable manner. This allows RSS readers, also known as feed readers or news aggregators, to automatically fetch and display the latest updates from multiple sources. By subscribing to RSS feeds, users can access a consolidated view of all the content they follow, saving them valuable time and effort.</p><p>Furthermore, RSS empowers users to have control over the content they consume. Unlike social media algorithms that prioritize certain posts and filter out others, RSS puts you in charge. You choose the sources you want to subscribe to, and you decide what you want to read. It eliminates the noise and distractions often associated with social media platforms, allowing you to focus on the content that truly matters to you.</p><p>So, whether you&apos;re a news junkie, a podcast enthusiast, or a blog lover, RSS is a powerful tool that enhances your online content consumption experience. It brings convenience, efficiency, and customization to the forefront, making it easier than ever to stay informed and engaged with the topics that interest you the most.</p><h2 id="how-can-you-leverage-the-power-of-rss">How Can You Leverage the Power of RSS?</h2><p>Now that you understand what RSS is and why it exists, let&apos;s explore how you can harness its power to your advantage. By using an RSS reader application, you can organize, filter, and customize your content consumption like never before.</p><p>An RSS reader acts as a centralized hub for all the feeds you subscribe to. It allows you to create folders, categorize your subscriptions, and prioritize the sources that matter most to you. You can even set up notifications to alert you when new articles are published, ensuring that you never miss a beat.</p><p>Imagine waking up in the morning and opening your RSS reader to find a neatly organized collection of articles from all your favorite blogs and news websites. With just a few clicks, you can skim through the headlines, read the summaries, and decide which articles you want to dive deeper into. No more wasting time visiting multiple websites or scrolling through endless social media feeds to find the content that interests you.</p><p>But the power of RSS doesn&apos;t stop there. With an RSS reader, you have the ability to filter and customize your content consumption to suit your specific needs and interests. Let&apos;s say you&apos;re a tech enthusiast who wants to stay up-to-date with the latest developments in artificial intelligence. You can create a folder specifically for AI-related feeds and subscribe to reputable sources in the field. This way, you&apos;ll receive only the most relevant and high-quality content on the topic, saving you time and effort.</p><p>Another advantage of using an RSS reader is the ability to prioritize your sources. Let&apos;s say you follow a dozen different blogs, but there are a few that consistently deliver exceptional content. With an RSS reader, you can mark these sources as a high priority, ensuring that their articles are always front and center in your feed. This way, you won&apos;t miss out on any valuable insights or thought-provoking ideas from your favorite authors.</p><p>Furthermore, an RSS reader allows you to stay on top of breaking news and time-sensitive information. By setting up notifications, you can receive instant alerts whenever a new article is published in your subscribed feeds. This feature is particularly useful for professionals who need to stay informed about industry trends or for individuals who want to be the first to know about important updates in their areas of interest.</p><p>In addition to all these benefits, using an RSS reader also enhances your privacy and reduces online distractions. Unlike social media platforms or news aggregators, an RSS reader doesn&apos;t track your browsing behavior or bombard you with targeted ads. It&apos;s a clean and focused way to consume content without being constantly bombarded by irrelevant information.</p><p>In conclusion, leveraging the power of RSS through an RSS reader can revolutionize the way you consume content. It allows you to streamline your information intake, customize your reading experience, and stay informed about the topics that matter most to you. So why not give it a try and experience the benefits for yourself?</p><h2 id="benefits-of-using-an-rss-applicationwith-suggestions">Benefits of Using an RSS Application - With Suggestions</h2><p>Using an RSS application offers numerous benefits. Firstly, it saves you time by eliminating the need to manually visit multiple websites. Instead of spending valuable minutes clicking through various sites, an RSS application aggregates all the desired content in one place. This allows you to efficiently browse through articles, blog posts, news updates, and more without the hassle of navigating multiple webpages.</p><p>Furthermore, an RSS application reduces information overload by allowing you to filter out irrelevant content and focus only on what truly matters to you. With the ability to customize your RSS feed, you can select specific topics, keywords, or even individual sources that align with your interests. By curating your feed, you can ensure that you receive only the most relevant and meaningful information.</p><p>There are many excellent RSS applications available today, each with its own unique features and advantages. One popular option is Feedly, a user-friendly application that allows you to easily organize and access your favorite websites and blogs. With its intuitive interface, Feedly makes it simple to add new sources, categorize content, and discover new articles based on your interests.</p><p>Inoreader is another noteworthy RSS application that offers powerful filtering options. With its advanced search capabilities, you can create complex queries to find specific articles or exclude certain keywords. Inoreader also provides the ability to save articles for later reading, ensuring that you never miss out on valuable content.</p><p>If you prefer a more nostalgic experience, The Old Reader might be the perfect choice for you. This RSS application aims to recreate the simplicity and charm of traditional feed readers. With its clean interface and straightforward functionality, The Old Reader allows you to focus solely on the content without any distractions.</p><p>Regardless of the RSS application you choose, one common advantage is seamless synchronization across devices. Whether you&apos;re using a desktop computer, laptop, tablet, or smartphone, your RSS feed will be automatically updated and synchronized. This ensures that you can access your curated content anytime, anywhere, and on any device.</p><p>In conclusion, utilizing an RSS application not only saves you time but also helps you manage information overload. With a wide range of options available, such as Feedly, Inoreader, and The Old Reader, you can find an RSS application that suits your preferences and enhances your browsing experience. So why wait? Start exploring the world of RSS today and discover a more efficient and personalized way to stay informed!</p><h2 id="finding-good-rss-feeds-to-follow">Finding Good RSS Feeds to Follow</h2><p>With thousands of websites and blogs out there, finding high-quality RSS feeds to follow can seem like a daunting task. However, with a little bit of effort and exploration, you can discover hidden gems that provide valuable and engaging content.</p><p>Start by considering the websites you already visit regularly. Many of them likely offer RSS feeds that you can subscribe to. Look for the RSS symbol or the word &quot;Subscribe&quot; on the site&apos;s homepage or in the navigation menu. Subscribing to these feeds will allow you to stay updated with the latest articles, news, and updates from your favorite websites without having to constantly visit each one individually.</p><p>But what if you want to discover new sources of information? Search engines can be a great tool for finding RSS feeds related to specific topics or industries. Simply type in your keywords or phrases of interest, followed by &quot;RSS feed,&quot; and you&apos;ll be presented with a list of relevant feeds that you can explore.</p><p>Another way to find interesting RSS feeds is by exploring social media platforms. Many content creators and influencers share their RSS feeds on platforms like Twitter, Facebook, and LinkedIn. By following these individuals or organizations, you can discover new and exciting sources of content that align with your interests.</p><p>When it comes to choosing which RSS feeds to subscribe to, don&apos;t be afraid to experiment and explore different options. Subscribe to a variety of sources that cover a wide range of topics or perspectives. This will ensure that you receive a well-rounded stream of content that keeps you informed and inspired.</p><p>Additionally, consider joining online communities or forums that discuss topics you&apos;re interested in. These communities often share and recommend RSS feeds that are highly regarded within their niche. By participating in these communities, you not only gain access to valuable discussions, but you also come across recommendations for feeds that you may have never discovered on your own.</p><p>Furthermore, many news aggregators and content curation platforms offer recommendations based on your interests and browsing history. These platforms use algorithms to analyze your preferences and suggest relevant RSS feeds that you might find interesting. Take advantage of these features to discover new feeds that align with your specific interests.</p><p>Lastly, don&apos;t forget to regularly review and prune your RSS feed subscriptions. As your interests evolve and change over time, some feeds may no longer provide the value they once did. By periodically evaluating and removing feeds that no longer align with your interests, you can ensure that your RSS feed collection remains fresh and relevant.</p><p>In conclusion, finding good RSS feeds to follow may initially seem overwhelming, but with a systematic approach and a willingness to explore, you can discover a wealth of valuable and engaging content. By leveraging the resources available to you, such as websites, search engines, social media platforms, online communities, and content curation platforms, you can curate a diverse collection of feeds that keep you informed, inspired, and connected to the topics that matter most to you.</p><h2 id="filtering-relevant-articles-from-an-rss-feed">Filtering Relevant Articles from an RSS Feed</h2><p>As you start subscribing to more and more feeds, it&apos;s important to develop a system for filtering out the noise and focusing on the articles that are most relevant to you. After all, the goal is to save time and optimize your content consumption.</p><p>One effective way to filter relevant articles from an RSS feed is by utilizing keyword filters. By specifying keywords that are of particular interest to you, you can ensure that articles containing those keywords are given priority in your feed. For example, if you are interested in technology news, you might set up filters for keywords such as &quot;technology,&quot; &quot;innovation,&quot; and &quot;startups.&quot; This way, articles that contain these keywords will be prominently displayed in your feed, allowing you to stay up-to-date with the latest developments in the tech industry.</p><p>Another useful filtering option is based on authors. If you have favorite authors or journalists whose work you admire, you can set up filters to display articles written by them. This ensures that you never miss an article from your favorite writers and allows you to easily access their insights and expertise.</p><p>In addition to keyword and author filters, many RSS applications also offer the ability to filter articles from specific websites. This is particularly helpful if you have a list of trusted sources that you prefer to follow. By setting up filters based on website URLs, you can ensure that articles from those sources are prominently displayed in your feed, while articles from other sources are filtered out.</p><p>Experimenting with these filters can greatly enhance your reading experience and save you valuable time. By customizing your feed to align with your specific interests, you can ensure that you receive the most relevant and engaging content.</p><p>It&apos;s worth noting that while filtering is a powerful tool, it&apos;s important to occasionally step out of your comfort zone and explore new topics and perspectives. Sometimes, the most unexpected articles can provide valuable insights and inspire new ideas. So, while filtering is a useful strategy, don&apos;t be afraid to occasionally venture into uncharted territory and broaden your horizons.</p><h2 id="avoid-these-pitfalls-with-rss">Avoid These Pitfalls with RSS</h2><p>While RSS is a powerful tool, it&apos;s important to be mindful of a few potential pitfalls. Firstly, be cautious of overwhelming yourself with too many subscriptions. It can be tempting to subscribe to every interesting-looking feed, but this can quickly lead to information overload. Be selective and prioritize quality over quantity.</p><p>When it comes to managing your RSS subscriptions, it&apos;s essential to strike a balance. On one hand, you want to stay informed and up-to-date with the latest news and content. On the other hand, subscribing to too many feeds can result in an overwhelming influx of information, making it difficult to keep up with everything.</p><p>One way to avoid this pitfall is to categorize your subscriptions. By organizing your feeds into different categories such as news, technology, sports, or entertainment, you can easily navigate through the content that matters most to you. This allows you to focus on specific topics without feeling overwhelmed by the sheer volume of information available.</p><p>Additionally, it&apos;s crucial to periodically review and revisit your subscriptions. Websites change, content strategies evolve, and some sources may no longer be relevant or reliable. Regularly clean up your feeds, removing sources that no longer provide value or align with your interests.</p><p>Another potential pitfall to be aware of is the risk of falling into an echo chamber. It&apos;s easy to get comfortable with the sources you trust and agree with, but it&apos;s important to expose yourself to different perspectives and opinions. By diversifying your RSS subscriptions and including sources that offer varying viewpoints, you can broaden your understanding of a topic and avoid confirmation bias.</p><p>Furthermore, it&apos;s worth mentioning the importance of verifying the credibility of the sources you subscribe to. With the rise of fake news and misinformation, it&apos;s crucial to be vigilant about the reliability and accuracy of the content you consume. Take the time to research the credibility of a source before adding it to your RSS feed, and be skeptical of sensationalized headlines or articles lacking proper citations.</p><p>In conclusion, while RSS is a valuable tool for staying informed and organized, it&apos;s essential to navigate it with caution. Avoid overwhelming yourself with too many subscriptions, periodically review and update your feeds, diversify your sources to avoid an echo chamber effect, and verify the credibility of the content you consume. By being mindful of these potential pitfalls, you can make the most out of RSS and enhance your information gathering experience.</p><h2 id="moving-forward-with-rss">Moving Forward with RSS</h2><p>Now that you have a comprehensive understanding of RSS and its potential, it&apos;s time to take action. Start by exploring different RSS reader applications and find one that suits your needs and preferences. Then, begin subscribing to feeds that align with your interests and goals.</p><p>Remember, RSS is about simplifying your online experience and allowing you to stay informed and organized. Embrace the power of RSS and take control of your content consumption. Start using RSS now, and you&apos;ll wonder how you ever lived without it!</p><p>Become a content connoisseur, a master organizer, and an informed individual. You must start using RSS now!</p>]]></content:encoded></item><item><title><![CDATA[Building an RSS Feed Aggregator with Go and ChatGPT [Part 1/3]]]></title><description><![CDATA[Implement an automatically generated weekly digest right into your mailbox with Golang and ChatGPT!]]></description><link>https://nooptoday.com/building-an-rss-feed-aggregator-with-go-and-chatgpt/</link><guid isPermaLink="false">64b1660e268a8900014b5720</guid><category><![CDATA[chatgpt]]></category><category><![CDATA[golang]]></category><category><![CDATA[programming]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Tue, 18 Jul 2023 13:26:06 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/07/cover-go-rss-1.png" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/07/cover-go-rss-1.png" alt="Building an RSS Feed Aggregator with Go and ChatGPT [Part 1/3]"><p>I&apos;ve recently quit my job. I am looking for new jobs but I have found no luck landing a backend developer job with my Node.js knowledge. So, I decided to learn other languages. I&apos;ve started the journey with Rust, it was enlightening but there are hardly any jobs for a junior Rust developer.</p><p>Enter Go, which has far more adaptation in the industry as a backend language compared to Node.js and Rust. Go is good for mainly two reasons. It is relatively fast ( faster than interpreted languages, a little slower than Rust, C, etc. ). It has really good developer ergonomics: fast compile times, easy syntax, and no async / await.</p><p>After learning the basics through <a href="https://go.dev/tour/welcome/1?ref=noop-today">A Tour of Go</a>, I&apos;ve decided to extend my understanding by building a real world application. Not the <a href="https://realworld-docs.netlify.app/?ref=noop-today">RealWorld</a> real-word, but the actual real world.</p><h2 id="the-idea">The Idea</h2><p>The application will aggregate new blog posts from various RSS feeds and notify the user with a newsletter of interesting posts. To make things interesting, the application will filter the posts according to:</p><ul><li>How much the post is related to the tags/topics the user is interested in,</li><li>The popularity of the post.</li></ul><p>The application then will generate summaries of each post using ChatGPT. And the newsletter will have a relatively simple design, such as:</p><pre><code class="language-bash">Hello {Subscriber.Name},

Here are some blog posts you might find interesting:

#ForEach Post in Posts
    {Post.Name} - by {Post.Author || Post.Website}
        {Post.SummaryByChatGPT}
#ForEachEnd


Click here -Unsubscribe Link- if you don&apos;t want to receive newsletters anymore</code></pre><p>My initial thought about the application structure on a high level is as follows:</p><figure class="kg-card kg-image-card"><img src="https://nooptoday.com/content/images/2023/07/rss-feed-aggregator.png" class="kg-image" alt="Building an RSS Feed Aggregator with Go and ChatGPT [Part 1/3]" loading="lazy" width="1262" height="1682" srcset="https://nooptoday.com/content/images/size/w600/2023/07/rss-feed-aggregator.png 600w, https://nooptoday.com/content/images/size/w1000/2023/07/rss-feed-aggregator.png 1000w, https://nooptoday.com/content/images/2023/07/rss-feed-aggregator.png 1262w" sizes="(min-width: 720px) 720px"></figure><p>The first few plans emerge from the diagram:</p><ul><li>Find some RSS sources,</li><li>Find a suitable package for getting RSS sources,</li><li>Find a DB driver package ( probably will use MongoDB ),</li><li>Find how to trigger cron jobs in Go ( I was using BullMQ in Node.js projects ),</li><li>Find an email composer and email sender package,</li></ul><p>Additionally, there seem to be a few endpoints I can add to the application:</p><ul><li>Classic sign-up / sign-in endpoints,</li><li>Newsletter management endpoints: subscribe / unsubscribe,</li><li>User interests management endpoint: PUT /user/interests</li></ul><p>To keep things simple, I will manage RSS sources via configuration files, and any change will require a system restart.</p><blockquote>Notice, I&apos;ve also used a single PUT request for user interests. This is again for simplicity. I am anticipating that users will have a few interests and I don&apos;t need to manage them via POST, DELETE endpoints. The list can be calculated on the client side.</blockquote><h2 id="the-plan">The Plan</h2><p>At first, all the implementations will be written in an unstructured way. I am sure most of the codes will look <em>ugly</em> for an experienced Go developer. But hey, my mission is to make the application <em>work</em> first, then fix the issues, refactor some parts, or do some performance optimizations.</p><p>In such scenarios, I always remind myself: Build something working first, others can be achieved later.</p><h2 id="setting-up-the-http-server">Setting up the HTTP Server</h2><p>I know building an HTTP server in Go is relatively simple. The built-in HTTP library is pretty darn good. For now, I will put everything inside the main.go file:</p><pre><code class="language-go">package main

import (
	&quot;log&quot;
	&quot;net/http&quot;
)

func main() {
	http.HandleFunc(&quot;/user/sign-in&quot;, signin)
	http.HandleFunc(&quot;/user/sign-up&quot;, signup)
	http.HandleFunc(&quot;/user/interests&quot;, interests)
	http.HandleFunc(&quot;/newsletter/subscribe&quot;, subscribe)
	http.HandleFunc(&quot;/newsletter/unsubscribe&quot;, unsubscribe)

	// Crash &amp; Log, in case anything happens during server startup
	log.Fatal(http.ListenAndServe(&quot;:3000&quot;, nil))
}

// User
func signup(w http.ResponseWriter, r *http.Request) {}
func signin(w http.ResponseWriter, r *http.Request) {}

// User Interests
func interests(w http.ResponseWriter, r *http.Request) {}

// Newsletter
func subscribe(w http.ResponseWriter, r *http.Request)   {}
func unsubscribe(w http.ResponseWriter, r *http.Request) {}
</code></pre><p>I believe that is all for the HTTP server. One thing I&apos;ve found missing compared to Express.js style HTTP servers is that you can&apos;t define the method on the router level. Handlers are responsible for all the HTTP methods ( GET, POST, PUT, etc. ).</p><p>The conventional way to handle this is to add switch statements even in the official documentation. I didn&apos;t want to add them ( because it would take up too much space on the blog post ).</p><h2 id="adding-the-rss-reader-library">Adding the RSS Reader Library</h2><p>There are various packages for reading &amp; parsing RSS feeds, but one package has significantly higher usage: <a href="https://github.com/mmcdole/gofeed?ref=noop-today">gofeed</a>. This package has a clean API. I&apos;ve tested it with the RSS feed from Noop Today.</p><pre><code class="language-go">package main

import (
	&quot;fmt&quot;
	&quot;github.com/mmcdole/gofeed&quot;
)

func main() {
    fp := gofeed.NewParser()
    feed, _ := fp.ParseURL(&quot;https://nooptoday.com/rss&quot;)

	for _, post := range feed.Items {
		fmt.Println(post.Title)
	}
    // ...
    // snip
}</code></pre><p>The program outputs:</p><pre><code class="language-bash">Lecture Notes: NLP with Deep Learning - 1
Top 5 Struggles of Backend Developers
Reduce Network Usage - With This 1 Simple Trick!
FeathersJS Vs NestJS - Compared in 3 Key Areas
Scalable Websocket Server Implemented by ChatGPT
How to Trace Websocket Events in NestJS - [Case Study]
Why Websockets are Hard To Scale?
Best Way to Create Dynamic Modules in NestJS
Using Custom Decorators in NestJS</code></pre><p>That was easier than I expected &#x1F605;.</p><p>While at it, I will add a way to get RSS sources from the configuration. I want to keep things as simple as possible. The configuration file will be a simple text file with each line containing an RSS source.</p><p>These will be the RSS sources.</p><pre><code class="language-bash">https://nooptoday.com/rss
https://dev.to/rss
https://hackernoon.com/feed
https://www.cdn.geeksforgeeks.org/feed</code></pre><p>With the following addition to the <code>main</code> function, the program can be given RSS sources from a configuration file: <code>rss_sources</code>.</p><pre><code class="language-go">func main() {
	readFile, _ := os.Open(&quot;rss_sources&quot;)
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var rssSources []string
	for fileScanner.Scan() {
		rssSources = append(rssSources, fileScanner.Text())
	}
	readFile.Close()

    fmt.Println(rssSources)
}</code></pre><h2 id="setting-up-the-database-connection">Setting up the Database Connection</h2><p>It was really easy to connect to the database. I have no idea about the <em>Context</em> object in Go. So, I omitted to create a context with timeout and handling the error cases. Which, I did in almost all parts of the project. But, those can be resolved later.</p><pre><code class="language-go">func main(){
    clientOpts := options.Client().ApplyURI(&quot;mongodb://localhost:27017/?connect=direct&quot;)
    client, _ := mongo.Connect(context.TODO(), clientOpts)
}</code></pre><p>I tested if things are working correctly with the following code:</p><pre><code class="language-go">func main()&#xA0;{
	clientOpts := options.Client().ApplyURI(&quot;mongodb://localhost:27017/?connect=direct&quot;)
    client, _ := mongo.Connect(context.TODO(), clientOpts)
    cursor, _ := client.Database(&quot;rss_aggregator&quot;).Collection(&quot;posts&quot;).Find(context.TODO(), bson.D{})
    defer cursor.Close(context.TODO())
    for cursor.Next(context.TODO()) {
        var result bson.D
        cursor.Decode(&amp;result)
        fmt.Println(result)
    }
}</code></pre><p>It works!</p><p>We have two missing parts in the project: a ChatGPT client and a mail client.</p><h2 id="writing-the-chatgpt-client">Writing the ChatGPT Client</h2><p>There will be only a single HTTP request going to the ChatGPT. There is no need to add another package to the project for this.</p><p>At first, I don&apos;t want to create a separate ChatGPT client. This allows me to keep things simple. Also, I am trying not to organize things prematurely.</p><pre><code class="language-go">func createBlogSummary(blogTitle string, blogContent string) string {
	// Send a request to ChatGPT to create a summary
}</code></pre><p>A function named <code>createBlogSummary</code> is enough for this use case. I specifically designed a prompt for this. If you are interacting with ChatGPT via API, you can add <code>system</code> prompts to your completion requests.</p><pre><code class="language-json">{
  &quot;model&quot;: &quot;gpt-3.5-turbo&quot;,
  &quot;messages&quot;: [
    {
      &quot;role&quot;: &quot;system&quot;,
      &quot;content&quot;: &quot;You are highly capable of writing articles and creating a short summary for them. Help the user for creating short summaries of articles.&quot;
    },
    {
        &quot;role&quot;: &quot;user&quot;,
        &quot;content&quot;: &quot;Provide a short summary of the following blog post: Title: ${blogTitle}, Content: ${blogContent}&quot;
    }
  ]
}</code></pre><p>Tested this prompt with this blog post&apos;s content ( up until this paragraph ). And it gave me a fairly good summary:</p><pre><code class="language-json">{
	&quot;id&quot;: &quot;chatcmpl-7dMPR2tsVa1QjDPY0QMN9e4abGKNL&quot;,
	&quot;object&quot;: &quot;chat.completion&quot;,
	&quot;created&quot;: 1689615289,
	&quot;model&quot;: &quot;gpt-3.5-turbo-0613&quot;,
	&quot;choices&quot;: [
		{
			&quot;index&quot;: 0,
			&quot;message&quot;: {
				&quot;role&quot;: &quot;assistant&quot;,
				&quot;content&quot;: &quot;This blog post discusses the author&apos;s decision to learn the programming language Go after facing difficulties finding a backend developer job with their knowledge in Node.js. They share their journey of building a real-world application using Go, specifically an RSS feed aggregator that generates summaries of blog posts using ChatGPT. The author outlines the initial plans for the application, including finding RSS sources, setting up a database connection, and integrating a ChatGPT client. They also discuss setting up an HTTP server using the built-in HTTP library in Go. The blog post concludes by mentioning the next steps, such as adding a mail client and finalizing the implementation of the ChatGPT client.&quot;
			},
			&quot;finish_reason&quot;: &quot;stop&quot;
		}
	],
	&quot;usage&quot;: {
		&quot;prompt_tokens&quot;: 986,
		&quot;completion_tokens&quot;: 132,
		&quot;total_tokens&quot;: 1118
	}
}</code></pre><p>Let&apos;s fill in the function body to send a request to ChatGPT API:</p><pre><code class="language-go">
func createBlogSummary(blogTitle string, blogContent string) string {
	chatGPTRequest := ChatGPTRequest{
		Model: ChatGPTTurbo,
		Messages: []ChatGPTRequestMessage{{
			Role:    ChatGPTSystem,
			Content: &quot;You are highly capable of writing articles and creating a short summary for them. Help the user for creating short summaries of articles.&quot;,
		}, {
			Role:    ChatGPTUser,
			Content: &quot;Provide a short summary of the following blog post: Title: &quot; + blogTitle + &quot; Content: &quot; + blogContent,
		}},
	}

	requestBody, err := json.Marshal(chatGPTRequest)

	if err != nil {
		log.Fatal(err)
	}

	token := os.Getenv(&quot;OPENAI_TOKEN&quot;)
	requestBytes := bytes.NewBuffer(requestBody)
	request, _ := http.NewRequest(&quot;POST&quot;, &quot;https://api.openai.com/v1/chat/completions&quot;, requestBytes)
	request.Header.Set(&quot;content-type&quot;, &quot;application/json&quot;)
	request.Header.Set(&quot;Authorization&quot;, &quot;Bearer &quot; + token)

	client := http.Client{}
	response, err := client.Do(request)

	if err != nil {
		fmt.Println(err)
		return &quot;&quot;
	}

	defer response.Body.Close()
	body, err := io.ReadAll(response.Body)
	if err != nil {
		fmt.Println(err)
	}

	return string(body)
}</code></pre><p>I didn&apos;t include the struct definitions for ChatGPTRequest, but you can imagine it is a simple struct with the fields in the request body.</p><p>Also, at first, I tried to send the request in the following form:</p><pre><code class="language-go">http.Post(&quot;https://api.openai.com/v1/chat/completions&quot;, &quot;application/json&quot;, requestBytes)</code></pre><p>But, apparently there is no way to set headers in this format. So, I used the <code>http.NewRequest</code> API, which allows for modifying headers before sending the request. </p><p>Okay, the last functioning part we need is a mail client.</p><h2 id="mail-client">Mail Client</h2><p>Surprise, surprise. I am running a self-hosted mail server. I can send emails from the server using SMTP. I am happy that <code>Go</code> has a built-in SMTP client. I used it according to the documentation and this is the function that will send newsletters to users:</p><pre><code class="language-go">func sendNewsletter(to string, content string) {
	server := os.Getenv(&quot;SMTP_SERVER&quot;)
	fromIdentity := os.Getenv(&quot;SMTP_FROM_IDENTITY&quot;)
	fromMail := os.Getenv(&quot;SMTP_FROM_MAIL&quot;)
	password := os.Getenv(&quot;SMPT_PASSWORD&quot;)

	msg := []byte(&quot;From: &quot; + fromIdentity + &quot; &quot; + fromMail + &quot;\r\n&quot; +
		&quot;To: &quot; + to + &quot;\r\n&quot; +
		&quot;Subject: Newsletter: Weekly Blog Summaries\r\n&quot; +
		&quot;\r\n&quot; + content + &quot;\r\n&quot;)

	err := smtp.SendMail(
		server+&quot;:587&quot;,
		smtp.PlainAuth(fromIdentity, fromMail, password, server),
		fromMail,
		[]string{to},
		msg)

	if err != nil {
		fmt.Println(err)
	}
}</code></pre><p>When I first tried to log environment variables they didn&apos;t show up &#x1F61E;. I had to add a package to the project to include environment variables from the <code>.env</code> file. I&apos;ve used <a href="https://github.com/joho/godotenv/?ref=noop-today">https://github.com/joho/godotenv/</a> package. It is very simple to use. You only need to add the following code at the beginning of the main function.</p><pre><code class="language-go">func main() {
	err := godotenv.Load()
	if err != nil {
		log.Fatal(&quot;Error loading .env file&quot;)
	}
}</code></pre><h2 id="summary">Summary</h2><p>Now, the project meets all the functional requirements:</p><ul><li>Users can sign-in/sign-up</li><li>Users can manage their interests and they can subscribe/unsubscribe to the newsletter</li><li>The project can read RSS feeds from a predefined list of RSS sources</li><li>The project can create summaries of blog posts using ChatGPT API</li><li>The project can send emails to users</li></ul><h3 id="what-is-next">What is next?</h3><p>In the following post, I need to merge all the functionalities to work together. The project is missing a user interface. Also one functional requirement is missing at the moment, which is running cron jobs in Go. For now, it didn&apos;t create a problem since the project is not ready, yet.</p><h3 id="help-wanted">Help wanted!</h3><p>I am doing this project to learn the Go language. Just so I have a functioning program doesn&apos;t mean it is written in a conventional way. I am sure there are &quot;Go way of doing things&quot; that I missed. Please let me know in the comments what could have been written better.</p>]]></content:encoded></item><item><title><![CDATA[Lecture Notes: NLP with Deep Learning - 1]]></title><description><![CDATA[What is the science behind ChatGPT? How can computers understand words? Word embedding and the Word2Vec algorithm are the starting blocks of NLP.]]></description><link>https://nooptoday.com/nlp-with-deep-learning-lecture-notes-1/</link><guid isPermaLink="false">64286520268a8900014b51fa</guid><category><![CDATA[nlp]]></category><category><![CDATA[chatgpt]]></category><category><![CDATA[gpt]]></category><category><![CDATA[word2vec]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Sat, 01 Apr 2023 20:30:29 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/04/nlp-cover-1.jpg" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/04/nlp-cover-1.jpg" alt="Lecture Notes: NLP with Deep Learning - 1"><p>I can&apos;t believe it has been four years since I finished college. To my surprise, I missed taking courses just for the sake of science.</p><p>Working at a company, putting your knowledge and effort to build something is excellent and satisfying. You also gain experience by doing that.</p><p>But curiosity is a strong human motive to learn something new. I think the latest advances in AI and the hype around ChatGPT and language models triggered my curiosity.</p><p>So, I decided to take open lectures by Standford University on YouTube. I wanted to keep track of my learning progress and share my lecture notes in this series. </p><div class="kg-card kg-callout-card kg-callout-card-grey"><div class="kg-callout-emoji">&#x1F64C;&#x1F3FB;</div><div class="kg-callout-text">Subscribe to join my journey or if you just want to follow along and learn something new.</div></div><h2 id="meaning-of-a-word">Meaning of a Word</h2><p>Even though I used ChatGPT before, I didn&apos;t know the science behind it. It all looked like magic to me, as it does to most people.</p><p>In the first lecture, I got really excited to see how we can translate words into a machine-understandable format. The lesson started with the following comic:</p><figure class="kg-card kg-image-card kg-card-hascaption"><img src="https://nooptoday.com/content/images/2023/04/care-less-1.png" class="kg-image" alt="Lecture Notes: NLP with Deep Learning - 1" loading="lazy" width="448" height="243"><figcaption>Source: <a href="https://languagelog.ldc.upenn.edu/nll/?p=21170&amp;ref=noop-today">https://languagelog.ldc.upenn.edu/nll/?p=21170</a></figcaption></figure><figure class="kg-card kg-image-card kg-card-hascaption"><img src="https://nooptoday.com/content/images/2023/04/care-less-2.png" class="kg-image" alt="Lecture Notes: NLP with Deep Learning - 1" loading="lazy" width="753" height="571" srcset="https://nooptoday.com/content/images/size/w600/2023/04/care-less-2.png 600w, https://nooptoday.com/content/images/2023/04/care-less-2.png 753w" sizes="(min-width: 720px) 720px"><figcaption>Source: <a href="https://languagelog.ldc.upenn.edu/nll/?p=21170&amp;ref=noop-today">https://languagelog.ldc.upenn.edu/nll/?p=21170</a></figcaption></figure><p>The comic is a mind-opener that makes you question how you give meanings to words.</p><p>Language constantly evolves, there are new words entering our lives and some words stop being used and some words gain new meanings!</p><p>Moreover, the meaning of a word also changes depending on the <strong>context</strong>.</p><h2 id="word-embedding">Word Embedding</h2><p>Word embedding is a large dimensional ( hundreds of dimensions ) space made up of word vectors. A word vector is the mathematical representation of the meaning of a word. </p><p>I&apos;ll explain how scientists turn words into mathematics in a minute. But first, let&apos;s look at an extremely simplified two-dimensional representation of a word embedding:</p><figure class="kg-card kg-image-card kg-card-hascaption"><img src="https://nooptoday.com/content/images/2023/04/Screenshot-2023-04-01-at-21.17.49.png" class="kg-image" alt="Lecture Notes: NLP with Deep Learning - 1" loading="lazy" width="748" height="728" srcset="https://nooptoday.com/content/images/size/w600/2023/04/Screenshot-2023-04-01-at-21.17.49.png 600w, https://nooptoday.com/content/images/2023/04/Screenshot-2023-04-01-at-21.17.49.png 748w" sizes="(min-width: 720px) 720px"><figcaption>Simplified Word Embedding</figcaption></figure><p>Of course, there are no labels such as age or gender in a real word embedding, because dimensions don&apos;t have to represent a specific thing.</p><p>The meaning of words is too broad to represent in a two-dimensional space. But having too many dimensions make the computation harder. A relatively sufficient word embedding can have 300 dimensions.</p><p>Even though we can&apos;t visualize a space with hundreds of dimensions in our heads, it is enough to understand why we need to have so many dimensions for representing words as vectors.</p><h2 id="word2vec-algorithm">Word2Vec Algorithm</h2><p>Word2Vec is one of the models you can use to translate words into a machine-readable format. Basically, it converts words into vectors.</p><p>I will try to explain how Word2Vec converts words into vectors, but honestly, the idea is all I can present to you. Because I didn&apos;t understand the mathematics behind it.</p><p>What do we want to achieve by converting a word to a vector? We want to create a word embedding in which semantically similar words are closer to each other.</p><p>The algorithm is based on the famous quote:</p><blockquote><em>You shall know a word by the company it keeps. - </em><a href="https://en.wikipedia.org/wiki/John_Rupert_Firth?ref=noop-today">John Rupert Firth</a></blockquote><p>The Word2Vec algorithm takes a large corpus ( body ) of text as input. Then tries to assign vectors to every unique word in the text. The value of the vector is adjusted iteratively to create a probability function P.</p><p><code>P(c|o) = Probability of word <strong>o</strong> appear in the context of center word <strong>c</strong></code></p><p>The probability function gives the probability of a word appearing in the context of another word. In each iteration, the vectors are adjusted and the probability function gives more accurate results.</p><p>The context words and the center word are defined as in the example:</p><p><code><strong>This</strong> <em>is a really big text</em> input you give to the algorithm.</code></p><p><code><em>This</em> <strong>is</strong><em> a really big text</em> <em>input</em> you give to the algorithm.</code></p><p><code><em>This</em> <em>is <strong>a</strong> really big text</em> <em>input you</em> give to the algorithm.</code></p><p><code><em>This</em> <em>is a <strong>really</strong> big text</em> <em>input you give</em> to the algorithm.</code></p><p>At each step, a word is selected as the center word and words within a certain radius of the center word are context words.</p><p>If the above sentence is the only input you give to the algorithm, you expect the probability of the word &quot;<em>big&quot;</em> appearing in the context of &quot;<em>text&quot;</em> to be very high.</p><p>And with a large enough dataset, this algorithm can create a word embedding in which words are closer to each other if they are semantically similar.</p><h2 id="results-analogy-example">Results: Analogy Example</h2><p>At the end of the lecture, there is a fascinating use case example for the algorithm.</p><p>Since words are described as vectors via the Word2Vec algorithm, you can apply vector operations on words!</p><p>Firstly, you can ask the algorithm: &quot;What are some similar words to the <strong>banana</strong>?&quot;. And the algorithm gives very accurate results such as &quot;coconut, mongo, bananas&quot;.</p><p>Once those vectors are calculated, this is just a question of finding the nearest points on a k-dimensional space. This is easily answered by running the <a href="https://www.ibm.com/topics/knn?ref=noop-today#:~:text=The%20k%2Dnearest%20neighbors%20algorithm%2C%20also%20known%20as%20KNN%20or,of%20an%20individual%20data%20point.">k-nearest neighbors</a> - <em>knn </em>- algorithm on the word embedding.</p><p>Secondly, you can add or subtract words from each other!</p><p>In the lecture, the following example is given:</p><p><code>King - Man + Woman = ?</code></p><p>And the algorithm replied correctly with the Queen!</p><p>So, with this algorithm, you can find analogies very easily.</p><h2 id="summary">Summary</h2><p>This was a great introduction lecture. If you have read it so far, I strongly suggest you go watch it on <a href="https://www.youtube.com/watch?v=rmVRLeJRkl4&amp;list=PPSV&amp;ref=noop-today">YouTube</a>.</p><p>I will be sharing a post for each lecture on the list, so stay tuned!</p>]]></content:encoded></item><item><title><![CDATA[Top 5 Struggles of Backend Developers]]></title><description><![CDATA[The struggle is real for backend developers. From scalability to security issues, there are tons of things backend developers take into account.]]></description><link>https://nooptoday.com/backend-development-top-5-struggles/</link><guid isPermaLink="false">6415be22268a8900014b4b6f</guid><category><![CDATA[backend]]></category><category><![CDATA[scalability]]></category><category><![CDATA[consistency]]></category><category><![CDATA[performance]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Fri, 31 Mar 2023 19:55:49 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/03/ai.jpg" medium="image"/><content:encoded><![CDATA[<blockquote>&quot;Web is an I/O device&quot;</blockquote><img src="https://nooptoday.com/content/images/2023/03/ai.jpg" alt="Top 5 Struggles of Backend Developers"><p>...as Uncle Bob describes.</p><p>I/O, Input / Output, <em>i.e </em>user input and server output.</p><p>Users want to access some data through applications or websites.</p><p>They want it to be available throughout the world over.</p><p>If the requirements are so simple, why is backend development so challenging? That is a great question I want to <em>hopefully</em> answer in this post<em>. </em>But more than that, this post should be a grumbling space for hardworking backend developers.</p><hr><p>Let&apos;s have a look at a simplistic diagram of a backend server.</p><figure class="kg-card kg-image-card kg-card-hascaption"><img src="https://nooptoday.com/content/images/2023/03/image.png" class="kg-image" alt="Top 5 Struggles of Backend Developers" loading="lazy" width="2000" height="604" srcset="https://nooptoday.com/content/images/size/w600/2023/03/image.png 600w, https://nooptoday.com/content/images/size/w1000/2023/03/image.png 1000w, https://nooptoday.com/content/images/size/w1600/2023/03/image.png 1600w, https://nooptoday.com/content/images/size/w2400/2023/03/image.png 2400w" sizes="(min-width: 720px) 720px"><figcaption>Simplistic Backend Server</figcaption></figure><p>Everything looks simple from the outside. But a lot is going on behind the scenes. In the end, it is called &quot;backend&quot; development for a reason.</p><p>The job of a backend developer is to make everything work seamlessly, at least from a user perspective.</p><p>So, here is my list of the top 5 struggles of backend developers.</p><h2 id="struggle-1-crashing-servers">Struggle 1: Crashing Servers!</h2><p>Backend developers need to always remind themselves:</p><blockquote>The server can go down at any point in time</blockquote><p>It is not only the server, unhandled rejections, network problems, and countless other things can cause application crashes.</p><p>This is a list of <a href="https://www.gremlin.com/community/tutorials/chaos-engineering-the-history-principles-and-practice/?ref=noop-today#fallacies-of-distributed-systems">fallacies of distributed systems:</a></p><ul><li>The network is reliable</li><li>Latency is zero</li><li>Bandwidth is infinite</li><li>The network is secure</li><li>Topology doesn&apos;t change</li><li>There is one administrator</li><li>Transport cost is zero</li><li>The network is homogeneous</li></ul><p>They are more than an inconvenience for backend development.</p><p>It is already hard to write correctly functioning programs. Now you also need to consider failure scenarios and make the application <strong>fault tolerant.</strong></p><figure class="kg-card kg-image-card"><img src="https://nooptoday.com/content/images/2023/03/image-1.png" class="kg-image" alt="Top 5 Struggles of Backend Developers" loading="lazy" width="390" height="449"></figure><h2 id="struggle-2-scalability">Struggle 2: Scalability</h2><p>There are various aspects of scalability, but let&apos;s keep things simple for the sake of this post.</p><p>A scalable application can continue to work even if the load increases significantly.</p><figure class="kg-card kg-image-card"><img src="https://nooptoday.com/content/images/2023/03/do-you-scale.jpeg" class="kg-image" alt="Top 5 Struggles of Backend Developers" loading="lazy" width="500" height="500"></figure><p>Servers have limited capacity and they can handle only a limited amount of requests.</p><ul><li>If you add more resources to the server it can handle more requests ( <em>vertical scaling</em> ).</li><li>If you add more servers to the system they can handle more requests ( <em>horizontal scaling ).</em></li></ul><p>Designing an application to be horizontally scalable, immediately changes the way things work.</p><p>As an example, most of the backend developers make race condition mistakes in scalable applications - <em>including myself. </em>In a scalable application, data can be read or modified by multiple servers. A typical example:</p><ul><li>Server A reads the data <code>{ foo: 1 }</code></li><li>Server B reads the data <code>{ foo: 1 }</code></li><li>Server A modifies the data and writes back: <code>{ foo: 1, bar: 2 }</code></li><li>Server B modifies the data and writes back: <code>{ foo: 1, baz: 2 &#xA0;}</code></li></ul><p>The result can be either <code>{ foo: 1, bar: 2 }</code> or <code>{ foo: 1, baz: 2 &#xA0;}</code> depending on which server writes back the data last. So, both servers are in a <a href="https://www.techtarget.com/searchstorage/definition/race-condition?ref=noop-today"><strong>race condition</strong>!</a></p><p>This was just an example, but there are a lot of things to consider when developing a horizontally scalable application.</p><h2 id="struggle-3-security">Struggle 3: Security</h2><p>Remember, a public server is accessible to everyone on the internet.</p><p>And, there are a lot of bad actors out there &#x1F977;.</p><p>Bad actors can craft special requests to extract information from a vulnerable server.</p><p>Go check out <a href="https://owasp.org/www-project-top-ten/?ref=noop-today">https://owasp.org/www-project-top-ten/</a> for what kind of sorcery those bad actors do to your servers.</p><p>Even legitimate users should be considered bad actors from a backend developer&apos;s perspective.</p><p>Legitimate users may abuse the service or send unexpected inputs to exploit the server&apos;s vulnerabilities. All user inputs should be sanitized to prevent unexpected inputs from legitimate users. Schema definitions, such as JSON Schema, are very helpful to sanitize user input.</p><p>Also, legitimate users are not allowed to do anything they want. The service needs to have a well-defined authorization system to prevent users from accessing forbidden resources.</p><p>The server might be vulnerable to a <em>Denial of Service</em> attack. Most DoS attacks can be prevented with rate limiting.</p><p>The list goes on, and the truth is no server can be 100% secure. Try to follow best practices as much as possible to secure your servers.</p><h2 id="struggle-4-consistency">Struggle 4: Consistency</h2><p>If a car is red, then it is red.</p><p>Other people should also tell you that car is red.</p><p>And, nobody should say that car isn&apos;t red.</p><p>Then tell me in the comments...</p><p> </p><figure class="kg-card kg-image-card kg-card-hascaption"><img src="https://nooptoday.com/content/images/2023/03/image-2.png" class="kg-image" alt="Top 5 Struggles of Backend Developers" loading="lazy" width="256" height="389"><figcaption>https://en.wikipedia.org/wiki/The_dress</figcaption></figure><p>...what is the color of this dress? Is it black &amp; blue or white &amp; gold? Whichever you choose, you would be surprised to see other people swear it is the other one! </p><p>You see, sometimes it is not very easy to come up with a single true answer to such a basic question.</p><p>In computer terms, this dress can be a document read from a cache, a slow replicate instance of a database, or an event-sourced database.</p><p>In all cases, the data might be obsolete. One server might read obsolete data while the other has access to a more recent version of the document.</p><h3 id="trade-off-consistency-and-performance">Trade-off: Consistency and Performance</h3><p>The problem with consistency is that retrieving the latest information may not be the optimum solution for most cases.</p><p>For example, if thousands of people try to access a popular tweet at the same time what do they see? The same tweet content, except the read counts, are probably way different from each other.</p><p>This is the result of a design decision. The engineers at Twitter decided to trade off between performance and consistency at the expense of showing obsolete read counts. This is a pretty good deal because the read counts are not a critical part of the tweets.</p><h3 id="more-about-consistency">More About Consistency</h3><p>Well, sometimes inconsistencies happen not because of design decisions but because of the way things work in software engineering.</p><p>Remember the previous example about the race condition? Well, how do you handle that? You might use locks but that can be costly ( in terms of time ). Some systems use optimistic locking to get better results. Some other systems use event sourcing and evade the problem completely.</p><p>Then there is the problem with client and backend consistency. The client database should be consistent with the actual data on the backend. Here is a nice article if you want to read more about consistency: <a href="https://medium.com/ssense-tech/handling-eventual-consistency-with-distributed-system-9235687ea5b3?ref=noop-today">https://medium.com/ssense-tech/handling-eventual-consistency-with-distributed-system-9235687ea5b3</a> </p><h2 id="struggle-5-performance">Struggle 5: Performance</h2><p>I think this one is pretty obvious. Everyone wants to develop blazingly fast backend applications.</p><p>But, that is not an easy task. From the language and framework selection to the algorithms used and the overall design of the system, almost everything has an impact on the performance.</p><p>I would say, the best course of action is:</p><ol><li>Use a performance testing tool such as <a href="https://k6.io/?ref=noop-today">k6</a></li><li>Don&apos;t waste time ultra-optimizing the application if it meets the criteria</li></ol><p>It is that simple...</p><p>...to give advice from a blog post &#x1F605;</p><p>In reality, measuring the application performance is somewhat complicated. Maybe you can easily measure the end-to-end performance of your endpoints etc.</p><p>But detecting the actual bottleneck in your application is hard without observability tools.</p><blockquote>In RESTful backend applications, a for loop or a basic algorithm in the code is rarely the bottleneck. Optimizing the database queries, network configurations, or API design usually have greater benefits.</blockquote><h2 id="summary">Summary</h2><p>The struggle is real for the backend developers. Share your struggles in the comments and show others they are not alone!</p><p>I hope you enjoyed this post, subscribe for more like this.</p>]]></content:encoded></item><item><title><![CDATA[Reduce Network Usage - With This 1 Simple Trick!]]></title><description><![CDATA[Want to reduce network usage greatly with a simple trick? You probably do it wrong, because I did it wrong the first time.]]></description><link>https://nooptoday.com/reduce-network-usage-with-simple-trick/</link><guid isPermaLink="false">64200802268a8900014b4daa</guid><category><![CDATA[compression]]></category><category><![CDATA[network]]></category><category><![CDATA[encryption]]></category><category><![CDATA[optimization]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Sun, 26 Mar 2023 20:30:30 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/03/less.jpg" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/03/less.jpg" alt="Reduce Network Usage - With This 1 Simple Trick!"><p>Making efficient use of the network is essential.</p><p>Efficient applications work faster and smoother, even in low network conditions.</p><p>Efficiency can be achieved in two stages:</p><ul><li>Design time</li><li>Runtime</li></ul><p>You might not want to mess with the design part if the application is already designed and actively running.</p><p>Then, there is this approach with application development: <strong>you should not optimize early.</strong></p><p>Runtime optimizations can reduce network usage to a desired level. If that is the case, optimizing the design can be considered an <strong>early optimization.</strong></p><h3 id="design-time-optimizations">Design Time Optimizations</h3><p>Design time optimizations for network usage relate to the overall design of how the application interacts with network resources.</p><p>For example, using GraphQL can reduce network usage for many applications. The reason is, GraphQL allows applications to fetch only the required amount of data and nothing else.</p><p>Some applications access the backend via BFF - <em><a href="https://blog.bitsrc.io/bff-pattern-backend-for-frontend-an-introduction-e4fa965128bf?ref=noop-today">Backend for Frontend</a>. </em>This can also help reduce network usage from the application point of view. The BFF can aggregate data from multiple servers and serve it to the application through a single API call.</p><p>These are some of the optimizations you can consider to reduce network usage in the design stage.</p><ul><li>Going beyond the REST, and creating optimized API endpoints</li><li>Keeping request/response schemas as little as possible</li><li>Using efficient data structures - <em>such as protocol buffers</em></li></ul><h3 id="runtime-optimizations">Runtime Optimizations</h3><p>Runtime optimizations relate to the smartness of the applications.</p><p>For example, a <strong>smart application</strong> shouldn&apos;t request the same resource twice. This can be achieved by implementing caching strategies</p><p>Caching is one of the most popular solutions to reduce network usage and network latency as well.</p><p>Despite its popularity, there is no ready-to-go solution for caching. Applications should use the correct caching strategies depending on the use case. In fact, caching is one of the hardest things in software development. Remember the famous quote:</p><blockquote>There are only two hard things in computer science: cache invalidation and naming things. - Phil Karlton</blockquote><p>With the increasing popularity of reactive programming, there are some clever tricks to reduce the number of network calls.</p><p>For example, imagine the application has a search box that shows the results in real-time. Instead of sending search requests after every time a letter is typed, the applications use a <strong>debounce </strong>operator. Debounce operator sends a search request after the user stops typing.</p><h3 id="the-simple-trick-compression">The Simple Trick: Compression</h3><p>Wait, did you make me read the article just to let me know about compression?</p><p>Don&apos;t get frustrated, I have a good reason to do so! Besides, I talk about other things, too. Bear with me &#x1F9F8;</p><p><strong>You might be using compression wrong.</strong></p><p>Compression is very simple to use because there are already well-known algorithms you can use out of the box.</p><p><strong>The problem is compressing the encrypted data.</strong></p><p>You should <strong>never compress the encrypted data</strong>, because you simply can&apos;t. The compressed data is hardly smaller than the original and the applications waste CPU cycles for compression &#x1F62E;.</p><p>Instead, you should <strong>always encrypt the compressed data.</strong></p><h4 id="how-to-tell-if-my-application-make-it-right">How to Tell If My Application Make It Right?</h4><p>Simply, toggle the compression on / off and compare the payload sizes. If the payload sizes are very close, then your application probably encrypts first.</p><h3 id="payload-comparison-compress-first-vs-encrypt-first">Payload Comparison: Compress First vs. Encrypt First</h3><p>To make comparison easier, I created a small example repository.</p><p>The example uses <code>aes-256-cbc</code> as the encryption method and <code>brotli</code> as the compression algorithm. You can also check the example in <a href="https://github.com/nooptoday/compression-encryption-order?ref=noop-today">GitHub</a>.</p><figure class="kg-card kg-code-card"><pre><code class="language-bash">[      resource ] http://jsonplaceholder.typicode.com/albums?userId=1
[      original ] 816
[  encryptFirst ] 746
[ compressFirst ] 244
[  % difference ] 67.29 reduction


[      resource ] http://jsonplaceholder.typicode.com/albums
[      original ] 9333
[  encryptFirst ] 7472
[ compressFirst ] 1723
[  % difference ] 76.94 reduction


[      resource ] http://jsonplaceholder.typicode.com/photos
[      original ] 1071472
[  encryptFirst ] 820717
[ compressFirst ] 98447
[  % difference ] 88.00 reduction
</code></pre><figcaption>https://github.com/nooptoday/compression-encryption-order</figcaption></figure><h3 id="why-compression-doesnt-work-after-encryption">Why Compression Doesn&apos;t Work After Encryption?</h3><p>The reason is simple. Compression algorithms try to represent the same data with fewer bytes.</p><p>They first try to identify repeating patterns or some kind of order in the original data. If the original data contains the word <code>lorem ipsum</code> 5 times, the compressed data replaces all recurring occurrences with references to the first occurrence.</p><p>The responsibility of encryption algorithms is to break recognizable patterns in the original data. They scramble the data so that it can&apos;t be understood by external actors. They create a high entropy representation of the original data, in cryptography terms.</p><p>Essentially, encrypted data loses its recognizable patterns and it becomes harder to compress.</p><p>A real-life equivalent could be a shopping list.</p><ul><li>Egg</li><li>Egg</li><li>Egg</li><li>Egg</li><li>Egg</li><li>Milk</li></ul><p>You can easily remember this list easily because it comes down to:</p><ul><li>5 x Egg</li><li>Milk</li></ul><p>But that is not possible with the following list:</p><ul><li>Egg</li><li>Milk</li><li>Bread</li><li>Apples</li><li>Lemons</li><li>Chocolate</li></ul><p>Even though there are the same amount of items in both lists, the second list can&apos;t be compressed. That is what encryption does to compression. </p><h3 id="further-reading">Further Reading</h3><p>Check out these articles for further reading:</p><ul><li><a href="https://www.geeksforgeeks.org/difference-between-data-encryption-and-data-compression/?ref=noop-today">https://www.geeksforgeeks.org/difference-between-data-encryption-and-data-compression/</a></li><li><a href="https://www.encryptionconsulting.com/education-center/encryption-and-compression/?ref=noop-today">https://www.encryptionconsulting.com/education-center/encryption-and-compression/</a></li></ul><p>As an additional note, there are some security concerns regarding compression. But that is out of the scope of this post.</p>]]></content:encoded></item><item><title><![CDATA[FeathersJS Vs NestJS - Compared in 3 Key Areas]]></title><description><![CDATA[FeathersJS and NestJS are not yet another javascript framework. They are the giants of backend development in Node.js. FeathersJS gives more flexibility to developers, while NestJS has a more structural approach.]]></description><link>https://nooptoday.com/feathersjs-vs-nestjs/</link><guid isPermaLink="false">640b2b58b9d0b00001c80b17</guid><category><![CDATA[nestjs]]></category><category><![CDATA[feathersjs]]></category><category><![CDATA[nodejs]]></category><category><![CDATA[backend]]></category><category><![CDATA[developer experience]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Sat, 11 Mar 2023 16:39:58 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/03/ironman.jpg" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/03/ironman.jpg" alt="FeathersJS Vs NestJS - Compared in 3 Key Areas"><p><a href="https://blog.feathersjs.com/introducing-feathers-5-the-api-and-real-time-application-framework-101ae2deaaeb?ref=noop-today">FeathersJS released its 5th major version, a.k.a Dove recently</a>, and I&apos;ve been watching the framework for some time. It is an excellent time to compare two backend framework giants, FeathersJS and NestJS. These are not your &quot;yet another javascript framework&quot; type of framework. They pave the way to backend development in the Node.js environment.</p><hr><p>In my opinion, Node.js provides a very powerful backend software development environment. A typical backend application accepts requests, executes queries on the database, and writes some files to disk. All of these tasks can be handled asynchronously in Node.js, meaning the server can handle many requests at the same time. Even though you can achieve the same or better results in other languages, it is easier to write asynchronous code in Javascript.</p><p>The struggle in the Node.js world is <strong>standardization.</strong> You have too many options to choose from, due to the large number of npm packages and the community size. You can find new javascript framework releases every other week, or someone in the community sharing their way of building applications. Don&apos;t get me wrong, I&apos;m happy to be in such an active community but this is a double-edged sword.</p><p>FeathersJS and NestJS both provide a structure for writing backend applications with their techniques. Both of them are capable of creating the same applications however their style differs a lot.</p><p>In this post, I will be comparing FeathersJS and NestJS from a developer experience perspective on key areas.</p><h2 id="application-structure">Application Structure</h2><h3 id="application-structurefeathersjs">Application Structure - FeathersJS</h3><p>To create a new FeathersJS application, execute this in your terminal: <code>npm create feathers</code></p><p>In the application creation process, FeathersJS CLI allows for many customizations such as:</p><ul><li>Use <code>TypeScript or JavaScript</code></li><li>Use <code>KoaJS or Express</code>,</li><li>Will you build a <code>REST application, Real-time application or all off them</code></li><li>Which package manager to use <code>npm, yarn, pnpm</code></li><li>What is your preferred schema definition format <code>TypeBox or JSON Schema</code></li><li>Which databases will you connect to <code>SQLite, MongoDB, PostgreSQL, MySQL/MariaDB, MicrosoftSQL</code></li><li>Select authentication mechanisms <code>Email + Password, Google, Facebook, Twitter, Github and Auth0</code></li></ul><figure class="kg-card kg-code-card"><pre><code class="language-treeview">feathers-app/
&#x251C;&#x2500;&#x2500; config/
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; custom-environment-variables.json
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; default.json
&#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; test.json
&#x251C;&#x2500;&#x2500; knexfile.ts
&#x251C;&#x2500;&#x2500; migrations/
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; 20230205130353_user.ts
&#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; 20230205130403_authentication.ts
&#x251C;&#x2500;&#x2500; package-lock.json
&#x251C;&#x2500;&#x2500; package.json
&#x251C;&#x2500;&#x2500; public/
&#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; index.html
&#x251C;&#x2500;&#x2500; readme.md
&#x251C;&#x2500;&#x2500; src/
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; app.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; authentication.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; channels.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; client.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; configuration.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; declarations.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; hooks/
&#x2502;&#xA0;&#xA0; &#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; log-error.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; index.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; logger.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; services/
&#x2502;&#xA0;&#xA0; &#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; index.ts
&#x2502;&#xA0;&#xA0; &#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; users/
&#x2502;&#xA0;&#xA0; &#x2502;&#xA0;&#xA0;     &#x251C;&#x2500;&#x2500; users.class.ts
&#x2502;&#xA0;&#xA0; &#x2502;&#xA0;&#xA0;     &#x251C;&#x2500;&#x2500; users.schema.ts
&#x2502;&#xA0;&#xA0; &#x2502;&#xA0;&#xA0;     &#x251C;&#x2500;&#x2500; users.shared.ts
&#x2502;&#xA0;&#xA0; &#x2502;&#xA0;&#xA0;     &#x2514;&#x2500;&#x2500; users.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; sqlite.ts
&#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; validators.ts
&#x251C;&#x2500;&#x2500; test/
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; app.test.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; client.test.ts
&#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; services/
&#x2502;&#xA0;&#xA0;     &#x2514;&#x2500;&#x2500; users/
&#x2502;&#xA0;&#xA0;         &#x2514;&#x2500;&#x2500; users.test.ts
&#x2514;&#x2500;&#x2500; tsconfig.json</code></pre><figcaption>FeathersJS application folder structure, automatically generated by CLI</figcaption></figure><p>FeathersJS application folder structure allows developers to easily decide where to put new files. It is well-structured and easily recognizable.</p><p>FeathersJS&apos;s preferred style of writing applications revolves around the idea of <strong>services</strong>. If you want to introduce new functionality in your application, you are most likely going to create a new <strong>service</strong> in FeathersJS.</p><p>Other than that, core functionalities such as authentication, routing, logging, and validation are well-defined in FeathersJS. You are very likely to use existing solutions provided by the framework with little configuration.</p><p>One thing that makes FeathersJS special is, it can <strong>generate its client code automatically.</strong> This is a unique feature I&apos;ve never heard before - <em>correct me if I am wrong on this.</em></p><p>Another special feature of FeathersJS is that it has the concept of <strong>channels</strong>. Which allows clients or other servers running your code to get notified about certain events. It is almost an in-app message broker on the server side. I&apos;m not sure about the scalability of channels though! This is a very neat feature considering FeathersJS calls itself: <a href="https://feathersjs.com/?ref=noop-today">The API and Real-time<br>Application Framework.</a></p><p>Least but not least, you can replace almost all 3rd party npm packages in FeathersJS to your taste. You might want to use another validation library or logging library, BUT you will never want to worry about your HTTP server library. FeathersJS HTTP server works fast and its <strong>performance is not dependent on the underlying HTTP server</strong>, read more about this here: <a href="https://feathersjs.com/guides/whats-new.html?ref=noop-today#lightning-fast-routing">Lightning Fast Routing</a></p><h3 id="application-structurenestjs">Application Structure - NestJS</h3><p>To create a new NestJS application, execute this on your terminal: <code>nest new my-project</code></p><p>NestJS doesn&apos;t provide many customizations through CLI but it has guides for customizations such as switching HTTP adapters.</p><figure class="kg-card kg-code-card"><pre><code class="language-treeview">nest-app/
&#x251C;&#x2500;&#x2500; README.md
&#x251C;&#x2500;&#x2500; nest-cli.json
&#x251C;&#x2500;&#x2500; package-lock.json
&#x251C;&#x2500;&#x2500; package.json
&#x251C;&#x2500;&#x2500; src/
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; app.controller.spec.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; app.controller.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; app.module.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; app.service.ts
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; main.ts
&#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; user/
&#x2502;&#xA0;&#xA0;     &#x251C;&#x2500;&#x2500; dto/
&#x2502;&#xA0;&#xA0;     &#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; create-user.dto.ts
&#x2502;&#xA0;&#xA0;     &#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; update-user.dto.ts
&#x2502;&#xA0;&#xA0;     &#x251C;&#x2500;&#x2500; entities/
&#x2502;&#xA0;&#xA0;     &#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; user.entity.ts
&#x2502;&#xA0;&#xA0;     &#x251C;&#x2500;&#x2500; user.controller.spec.ts
&#x2502;&#xA0;&#xA0;     &#x251C;&#x2500;&#x2500; user.controller.ts
&#x2502;&#xA0;&#xA0;     &#x251C;&#x2500;&#x2500; user.module.ts
&#x2502;&#xA0;&#xA0;     &#x251C;&#x2500;&#x2500; user.service.spec.ts
&#x2502;&#xA0;&#xA0;     &#x2514;&#x2500;&#x2500; user.service.ts
&#x251C;&#x2500;&#x2500; test/
&#x2502;&#xA0;&#xA0; &#x251C;&#x2500;&#x2500; app.e2e-spec.ts
&#x2502;&#xA0;&#xA0; &#x2514;&#x2500;&#x2500; jest-e2e.json
&#x251C;&#x2500;&#x2500; tsconfig.build.json
&#x2514;&#x2500;&#x2500; tsconfig.json
</code></pre><figcaption>NestJS application folder structure, automatically generated by CLI</figcaption></figure><p>NestJS has a very modular project structure which is also reflected in its folder structure.</p><p>NestJS&apos;s preferred style of writing applications revolves around the idea of <strong>modules</strong>. Modules contain providers and controllers, they can import other modules or export their providers, and they are configurable, read more about it in the previous post: <a href="https://nooptoday.com/dynamic-modules-in-nestjs/">Best Way to Create Dynamic Modules in NestJS</a>.</p><p>The business logic of a NestJS application usually resides in <strong>services</strong>. A service can use other services via <strong>dependency injection</strong>.</p><p>Implementation details of the NestJS framework are mostly hidden from the developers. <strong>NestJS exposes most of its functionality via Typescript decorators</strong>. If you are into code aesthetics you can prefer this style of coding, and read more about it in the previous post: <a href="https://nooptoday.com/custom-decorators-in-nestjs/">Custom Decorators in NestJS</a></p><p>Since NestJS doesn&apos;t allow much customization it is easy for other NestJS developers to adapt your project easily.</p><p>A unique approach NestJS brings to backend application development is using reactive programming for most things. You can see <strong>Observables being used in HTTP requests, events, commands anything you name.</strong> This might be an innovative approach but in my experience, observables cause a performance drop in NestJS applications.</p><p>Similar to FeathersJS, core functionalities are well-defined and you can find resources for whatever you want to do in a NestJS application. NestJS can be considered a little more mature compared to FeathersJS.</p><p>NestJS is also customizable but due to the way adapters are written, sometimes using a different package can cause a loss of functionality. For example, it is known that <strong>you lose functionality by switching from Express Adapter to Fastify Adapter,</strong> but this is a minor inconvenience for most developers.</p><p>Last but not least, <strong>NestJS CLI is capable of managing a monorepo project structure.</strong> Which is very helpful if you are doing <strong>microservices</strong> with NestJS.</p><h2 id="customization-options">Customization Options</h2><h3 id="customization-optionsfeathersjs">Customization Options - FeathersJS</h3><p>FeathersJS is intentionally made to be very customizable. There are two reasons for that.</p><p>Firstly, if you are already developing your backend application in ExpressJS, you can just switch ExpressJS with FeathersJS and your project will work just fine. From there on, you can make the full switch bit by bit. This allows an easier migration path for projects that use ExpressJS.</p><p>Secondly, FeathersJS doesn&apos;t want to have opinions about how you implement your application. Rather, it aims to lay out best practices and its own implementations for them. It has good solutions that work well, and you can use them whenever you want to.</p><h3 id="customization-optionsnestjs">Customization Options - NestJS</h3><p>NestJS is intentionally made to be less customizable. </p><p>NestJS aims to be the framework for enterprise applications. Enterprises don&apos;t like custom solutions. Custom solutions are costly to enterprise because it requires the knowledge of the person who implemented them. Developers can come and go, the project shouldn&apos;t be affected by this.</p><p>Also, we shouldn&apos;t forget NestJS is heavily inspired by AngularJS and it is also one of the less customizable frameworks for frontend.</p><p>BUT, this doesn&apos;t mean you can&apos;t customize your project or every other NestJS developer can immediately start working on existing NestJS projects. The reality is you can customize, but you shouldn&apos;t do it. Even if you do leave tracks of the NestJS railway, you will probably go back to the track for using some other functionality to work. Let me make it clear with a simple example:</p><p>You can opt out of the class-validator library in NestJS and use AJV for validation. It works without causing you problems but now you can&apos;t use automatic OpenAPI generation in NestJS.</p><h2 id="community-and-documentation">Community and Documentation</h2><h3 id="community-and-documentationfeathersjs">Community and Documentation - FeathersJS</h3><p>FeathersJS has a small, helpful, and welcoming community. You can join the community in Discord. One thing very noticeable on the FeathersJS website is the <a href="https://feathersjs.com/ecosystem/?ref=noop-today">Ecosystem</a> page where community-driven FeathersJS packages are promoted!</p><p>FeathersJS documentation walks you through everything you need in the framework. It is simple and clear. There are a few amounts of blog posts or online courses for FeathersJS, but I think numbers will go up after the latest release. David Luecke and other core maintainers are putting a lot of effort into the framework for some time, and I hope they get more recognition in the Node.js ecosystem.</p><h3 id="community-and-documentationnestjs">Community and Documentation - NestJS</h3><p>NestJS has a large, helpful, and welcoming community. You can join the community in Discord. NestJS has official online courses <a href="https://courses.nestjs.com/?ref=noop-today">that can be purchased on its website</a>.</p><p>I don&apos;t know how popular is their course but documentation on their website can get you through most of the things. To solve some problems you might need help from StackOverflow or GitHub issues but other than that, I think documentation and examples are enough. Can&apos;t skip here without mentioning <a href="https://www.udemy.com/course/nestjs-zero-to-hero/?ref=noop-today">NestJS Hero to Zero</a> online course by Ariel Weinberger, which I&apos;ve completed and found helpful.</p><p>I haven&apos;t had time to review the new <a href="https://devtools.nestjs.com/login?ref=noop-today">Developer Tools</a> by NestJS, but it looks like a very helpful tool. It is a paid tool by NestJS but at a low price. I think it would be much better if they released it for free at least initially.</p><h2 id="summary">Summary</h2><p>You can build applications of any size and functionality whichever you choose. To choose between FeathersJS and NestJS means choosing between two different code styles. In my opinion, the development process in FeathersJS feels more fitting ( natural ) to the Node.js environment, but, I also understand the reason why NestJS is preferred by enterprise applications.</p><p>Mad respect to both David Luecke and Kamil Mysliwiec and all the other contributors, and maintainers of both projects for creating two of the strongest backend frameworks for Node.js.</p><p>If you have experience in either of the frameworks, I would like to know what you think, please share your experiences and opinions in the comment section.</p><p>I hope you learned something new, have a good one!</p>]]></content:encoded></item><item><title><![CDATA[Scalable Websocket Server Implemented by ChatGPT]]></title><description><![CDATA[Learn how to create scalable websocket server using consistent hashing method in Node.js, with the help of ChatGPT!]]></description><link>https://nooptoday.com/scalable-websockets-by-chatgpt/</link><guid isPermaLink="false">63b9351be43b07000105ec35</guid><category><![CDATA[websocket]]></category><category><![CDATA[chatgpt]]></category><category><![CDATA[nodejs]]></category><category><![CDATA[hashring]]></category><category><![CDATA[scalable system]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Wed, 01 Feb 2023 07:30:00 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/03/piano.jpg" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/03/piano.jpg" alt="Scalable Websocket Server Implemented by ChatGPT"><p>In the previous post <a href="https://nooptoday.com/why-websockets-are-hard-to-scale/">Why Websockets are Hard to Scale</a> we talked about the problems with scaling websockets and some of the existing solutions as well. The post mentioned using a load-balancing technique called consistent hashing, which is a very elegant solution to this problem. I promised you a post about how to implement a scalable websocket server with consistent hashing.</p><p>Well, the day has come, take your coffee and follow along. Special thanks to ChatGPT, almost all of the code you will find in this post is written by ChatGPT. I only made some adjustments, so the code actually works.</p><hr><p>The plan is to use code generated by ChatGPT, and do as few modifications as possible on it. If you&apos;ve read the previous post I&apos;m sure you have an outline of what we will do. But to remind you again here are steps for our implementation:</p><ul><li>Create a websocket server</li><li>Create a hashring with server addresses, update when necessary</li><li>Retrieve the list of servers, and keep it updated for all servers</li><li>Redirect clients to the correct servers</li><li>Extra: Create a client that can redirect itself</li></ul><h3 id="new-favorite-ide-unlocked-the-prompter">New favorite IDE unlocked: The Prompter</h3><p>I was reluctant to test out the ChatGPT but it turns out to be very helpful, intelligent enough to give me code blocks that I can work on but still dumb enough to take my job ^^. Whether we like it or not, it is already here. As with all other technologies, if you embrace them sooner than later, you can use them to your advantage. At least that is what I realized from my experience.</p><blockquote>ChatGPT can be challenging to work with at times, but with practice and by using more specific prompts, it is possible to unlock its full potential. I recommend you to have a look at <a href="https://github.com/f/awesome-chatgpt-prompts?ref=noop-today">awesome-chatgpt-prompts</a> by my dear colleague, Fatih Kadir Ak&#x131;n, for inspiration on how to effectively communicate with ChatGPT.</blockquote><hr><h2 id="how-to-better-use-chatgpt-for-code-generation">How to Better Use ChatGPT for Code Generation</h2><p>Since ChatGPT will not create a fully completed project for us, each implementation step covers a single concept. Therefore we can get a working code block for each step and glue them together!</p><h3 id="hashring-implementation">Hashring Implementation</h3><p>I&apos;ve started with the hashring and asked ChatGPT to create me a hashring implementation in TypeScript, with support for virtual nodes. The first output from ChatGPT seemed okay but it used a <code>&gt;</code> comparison for strings. Also, the hash algorithm didn&apos;t create random and even results. First, I asked for fixing the bug with <code>&gt;</code> comparison. Then, I wanted it to use a hash function with better distribution.</p><pre><code class="language-typescript">import { createHash } from &apos;crypto&apos;;

class HashRing {
  private nodes: string[];
  private ring: { [key: string]: string };
  private replicas: number;

  constructor(nodes: string[], replicas: number) {
    this.nodes = nodes;
    this.replicas = replicas;
    this.ring = {};

    // Create a hash for each node and add it to the ring
    for (const node of nodes) {
      for (let i = 0; i &lt; replicas; i++) {
        const hash = createHash(&apos;sha1&apos;)
          .update(node + &apos;:&apos; + i)
          .digest(&apos;hex&apos;);
        this.ring[hash] = node;
      }
    }

    // Sort the keys in the ring to ensure that they are in clockwise order
    this.sortedKeys = Object.keys(this.ring).sort((a, b) =&gt;
      a.localeCompare(b, undefined, { numeric: true })
    );
  }

  // Get the node responsible for a given key
  getNode(key: string): string {
    const hash = createHash(&apos;sha1&apos;)
      .update(key)
      .digest(&apos;hex&apos;);
    const keys = this.sortedKeys;

    // Find the first node with a position greater than the hash
    let index = keys.findIndex((k) =&gt; k.localeCompare(hash, undefined, { numeric: true }) &gt; 0);

    // If no such node is found, the key belongs on the first node
    if (index === -1) {
      index = 0;
    }

    return this.ring[keys[index]];
  }
}</code></pre><p>With that complete, I copied the code into <code>hashring.ts</code> and exported the class. Whenever the server list is updated, this class will be recreated. Then we can iterate through all the connected sockets and send them a redirect message if they are not supposed to be on this server.</p><p>The full conversation with hashring implementation: <a href="https://sharegpt.com/c/kxhBEp6?ref=noop-today">https://sharegpt.com/c/kxhBEp6</a></p><h3 id="websocket-server-implementation">Websocket Server Implementation</h3><p>Next up, I requested ChatGPT to create a Websocket server. It immediately created a Hello World example using <code>ws</code> library.</p><pre><code class="language-typescript">import * as WebSocket from &apos;ws&apos;;

const wss = new WebSocket.Server({ port: 8080 });

wss.on(&apos;connection&apos;, (ws: WebSocket) =&gt; {
  ws.on(&apos;message&apos;, (message: string) =&gt; {
    console.log(`Received message =&gt; ${message}`);
  });

  ws.send(&apos;Hello! Message From Server&apos;);
});
</code></pre><p>ChatGPT has a tendency not to use default imports, but it is easy to fix. With the basic server setup, I requested ChatGPT to add a feature for redirecting clients based on hashring algorithm.</p><pre><code class="language-typescript">import * as crypto from &apos;crypto&apos;;

const serverList = [&apos;server1&apos;, &apos;server2&apos;, &apos;server3&apos;];

function getServer(clientIp: string) {
  const hash = crypto.createHash(&apos;md5&apos;).update(clientIp).digest(&apos;hex&apos;);
  const index = parseInt(hash, 16) % serverList.length;
  return serverList[index];
}

const clientIp = &apos;192.168.0.1&apos;;
const server = getServer(clientIp);
console.log(`Redirecting client to ${server}`);
</code></pre><p>But my request was a little unclear, so the output had a general idea but it was missing server and client interaction. So, I wanted ChatGPT to merge two outputs. After a few prompts, it created a working example.</p><pre><code class="language-typescript">import * as WebSocket from &apos;ws&apos;;
import * as crypto from &apos;crypto&apos;;

const serverList = [&apos;server1&apos;, &apos;server2&apos;, &apos;server3&apos;];

function getServer(clientIp: string) {
  const hash = crypto.createHash(&apos;md5&apos;).update(clientIp).digest(&apos;hex&apos;);
  const index = parseInt(hash, 16) % serverList.length;
  return serverList[index];
}

const wss = new WebSocket.Server({ port: 8080 });

wss.on(&apos;connection&apos;, (ws: WebSocket) =&gt; {
  const clientIp = ws.upgradeReq.connection.remoteAddress;
  const server = getServer(clientIp);
  console.log(`Redirecting client ${clientIp} to ${server}`);

  ws.send(`You have been redirected to ${server}`);

  ws.on(&apos;message&apos;, (message: string) =&gt; {
    console.log(`Received message from ${clientIp} =&gt; ${message}`);
  });
});
</code></pre><p>The example was looking OK, but sending a plain text message to clients is not very flexible. So, I asked ChatGPT to use a more manageable message body. Also, it was sending a redirect message whether the client was on the correct server or not, so I wanted ChatGPT to fix that.</p><pre><code class="language-typescript">import * as WebSocket from &apos;ws&apos;;
import * as crypto from &apos;crypto&apos;;

const serverList = [&apos;server1&apos;, &apos;server2&apos;, &apos;server3&apos;];

function getServer(clientIp: string) {
  const hash = crypto.createHash(&apos;md5&apos;).update(clientIp).digest(&apos;hex&apos;);
  const index = parseInt(hash, 16) % serverList.length;
  return serverList[index];
}

const wss = new WebSocket.Server({ port: 8080 });

wss.on(&apos;connection&apos;, (ws: WebSocket) =&gt; {
  const clientIp = ws.upgradeReq.connection.remoteAddress;
  const correctServer = getServer(clientIp);
  console.log(`Client ${clientIp} should be connected to ${correctServer}`);

  if (ws.upgradeReq.headers.host !== correctServer) {
    console.log(`Redirecting client ${clientIp} to ${correctServer}`);

    const message = {
      type: &apos;redirect&apos;,
      server: correctServer,
    };
    ws.send(JSON.stringify(message));

    ws.on(&apos;message&apos;, () =&gt; {});
  } else {
    console.log(`Client ${clientIp} is already connected to ${correctServer}`);

    ws.on(&apos;message&apos;, (message: string) =&gt; {
      console.log(`Received message from ${clientIp} =&gt; ${message}`);
    });
  }
});
</code></pre><p>Voila! This looks great but there is still more work to do. For example, the server list in this example is static, and we need to manually replace the <code>getServer</code> function with the <code>getNode</code> method from <code>Hashring</code>. One amazing side note, I asked the ChatGPT if it can create a client-side example that can interact with this server. And it did, it is a very intelligent tool that will amaze me for a long time.</p><p>The full conversation with the websocket server and websocket client implementation: <a href="https://sharegpt.com/c/pIc7amA?ref=noop-today">https://sharegpt.com/c/pIc7amA</a></p><pre><code class="language-typescript">import * as WebSocket from &apos;ws&apos;;

let ws = new WebSocket(&apos;ws://localhost:8080&apos;);

ws.on(&apos;open&apos;, () =&gt; {
  console.log(&apos;Connected to WebSocket server&apos;);
});

ws.on(&apos;message&apos;, (data: string) =&gt; {
  const message = JSON.parse(data);
  if (message.type === &apos;redirect&apos;) {
    console.log(`Received redirect message. Disconnecting from current server and connecting to ${message.server}`);
    ws.close();
    ws = new WebSocket(`ws://${message.server}:8080`);
  } else {
    console.log(`Received message =&gt; ${data}`);
  }
});

ws.on(&apos;close&apos;, () =&gt; {
  console.log(&apos;Disconnected from WebSocket server&apos;);
});
</code></pre><blockquote>Another important thing to note: Code examples from ChatGPT are not flawless, and I don&apos;t think they were meant to be. Sometimes they are easily noticable by IDEs or humans, but sometimes ChatGPT does something very unusual and doesn&apos;t work in any way. In that case, you can always ask it to fix its own code, and to my surprise again, it does a good job at it, too!</blockquote><h3 id="implementing-service-registry">Implementing Service Registry</h3><p>To create a hashring, we need addresses of all the available servers. This might sound simple, but it is really difficult to implement such things on the servers. You have to consider failure scenarios and update notifications etc. That is out of the scope of this post, but as a simpler solution, I was ready to use a 3rd party service registry solution. The problem is, I&apos;ve never used a service registry before.</p><p>At first, I asked ChatGPT to create a service registry implementation in TypeScript. It created a solution using Redis, but the code looked too complicated to me and I didn&apos;t want to use it. Instead, I&apos;ve found myself having a conversation with ChatGPT about which Service Registry Tools are there, what are their differences, etc. </p><blockquote><strong>Me: </strong>can you compare eureka, zookeeper and consul<br><br><strong>ChatGPT: </strong>Sure! <br>- Eureka, Zookeeper, and Consul are all tools that can be used for service discovery in a distributed environment.<br><br>- Eureka is a REST-based service that is primarily used in the Java ecosystem for locating services for the purpose of load balancing and failover of middle-tier servers.<br><br>- Apache Zookeeper is a distributed coordination service that enables distributed applications to achieve high availability. Zookeeper helps to coordinate distributed applications by providing a central place for them to store and access shared data.<br><br>- Consul is a distributed, highly available, and data center aware tool that can be used for service discovery, configuration, and orchestration. It is written in the Go programming language and provides a HTTP API for interacting with it.<br><br>- In summary, Eureka is primarily used in the Java ecosystem, Zookeeper is a distributed coordination service, and Consul is a tool that can be used for service discovery, configuration, and orchestration.</blockquote><p>Then I asked about simple examples using each one of the tools. In the end, <code>Consul</code> seemed like a good solution for this purpose and as a bonus, it was written in <code>Go</code>, so I gave it a try.</p><p><code>Consul</code> implementation from ChatGPT didn&apos;t work. Even though I asked ChatGPT to fix the code, it didn&apos;t work. I didn&apos;t want to debug and fix it because I was wanting to use as much code from ChatGPT. The problem was much more likely due to breaking changes from the npm package ( ChatGPT doesn&apos;t have up-to-date information on everything ).</p><p>So, what did I do? I went back to square one and asked for a Redis implementation, but this time I asked for Service Discovery instead of Service Registry. Apparently, I was asking for something slightly different - <a href="https://stackoverflow.com/questions/44540545/microservices-what-is-difference-between-service-registry-and-service-discovery?ref=noop-today">related SO post</a>.</p><pre><code class="language-typescript">import Redis from &apos;ioredis&apos;;

const redis = new Redis({ host: &apos;127.0.0.1&apos; });
const updateInterval = 5; // in seconds
const heartbeatInterval = 10; // in seconds

const serviceList: Set&lt;string&gt; = new Set();
const heartbeatIntervals: { [instance: string]: NodeJS.Timeout } = {};

async function registerServiceInstance(instance: string) {
  await redis.sadd(&apos;service-name&apos;, instance);
  await redis.set(instance, &apos;alive&apos;, &apos;EX&apos;, heartbeatInterval * 2);
  heartbeatIntervals[instance] = setInterval(() =&gt; {
    redis.set(instance, &apos;alive&apos;, &apos;EX&apos;, heartbeatInterval * 2);
  }, heartbeatInterval * 1000);
}

async function deregisterServiceInstance(instance: string) {
  await redis.srem(&apos;service-name&apos;, instance);
  await redis.del(instance);
  clearInterval(heartbeatIntervals[instance]);
  delete heartbeatIntervals[instance];
}

async function removeStaleServiceInstances() {
  const instances = await redis.smembers(&apos;service-name&apos;);
  for (const instance of instances) {
    const ttl = await redis.ttl(instance);
    if (ttl === -2 || ttl === -1) {
      await redis.srem(&apos;service-name&apos;, instance);
    }
  }
}

async function updateServiceList() {
  const instances = await redis.smembers(&apos;service-name&apos;);
  serviceList.clear();
  for (const instance of instances) {
    serviceList.add(instance);
  }
}

setInterval(removeStaleServiceInstances, updateInterval * 1000);
setInterval(updateServiceList, updateInterval * 1000);
</code></pre><p>In addition to raw service discovery implementation, I also asked for automatic updates and removing stale services. And it did a great job at creating them, too.</p><p>The full conversation with service discovery implementation: <a href="https://sharegpt.com/c/bGUAItM?ref=noop-today">https://sharegpt.com/c/bGUAItM</a></p><h2 id="putting-it-all-together-scalable-websocket-server">Putting it All Together: Scalable Websocket Server</h2><p>So far we&apos;ve implemented:</p><ul><li>Hashring, for determining which server a user belongs to,</li><li>Service Discovery to keep the list of available servers,</li><li>Websocket Server for clients to connect,</li><li>Websocket API for re-routing clients when necessary,</li><li>Websocket Client to connect our servers</li></ul><p>with the help of ChatGPT. We could of course implement these ourselves, but there is nothing wrong with getting help and speeding up the process. As far as you know what you are asking for and you can verify the code works as intended, ChatGPT only speeds up our development times. Also, it is very fun to work with.</p><p>Now we have all the pieces in our hands, we can just glue them together! GitHub repository with the final project and ChatGPT conversations: <a href="https://github.com/nooptoday/chatgpt-scalable-websockets?ref=noop-today">https://github.com/nooptoday/chatgpt-scalable-websockets</a></p><h3 id="results">Results</h3><p>You can clone the repository and test it for yourself! Here is a terminal output from running instances:</p><pre><code class="language-markdown">node1	| [node1] - [client_size] -&gt; [1884]
node2	| [node2] - [client_size] -&gt; [2237]
node3	| [node3] - [client_size] -&gt; [1879]</code></pre><p>6000 connections are sent to node1 initially, and clients are redirected to other nodes. In an ideal world, we would expect to see something like:</p><pre><code class="language-markdown">node1	| [node1] - [client_size] -&gt; [2000]
node2	| [node2] - [client_size] -&gt; [2000]
node3	| [node3] - [client_size] -&gt; [2000]</code></pre><p>You can play with the number of virtual nodes, which hash function to use or which parameter from the client to hash, and see how the results change.</p><p>If you have any questions about the implementation details you can ask here in the comments or create an issue in the GitHub repository. If you are asking, why there is a need for implementing such a solution, you can read the previous post that led to this implementation: <a href="https://nooptoday.com/why-websockets-are-hard-to-scale/">Why Websockets are Hard to Scale</a></p><p>That is it for this post, I hope you enjoyed and learned something new, let me know what you think in the comments!</p><p><em>Update 12/03/2023: Fix grammar and spelling mistakes.</em></p>]]></content:encoded></item><item><title><![CDATA[How to Trace Websocket Events in NestJS - [Case Study]]]></title><description><![CDATA[How do you trace websocket events in NestJS? There is no conventional way for it. Lets implement our solution with a custom WebSocketAdapter!]]></description><link>https://nooptoday.com/how-to-trace-websocket-events-in-nestjs/</link><guid isPermaLink="false">63b17181e43b07000105e8ae</guid><category><![CDATA[nestjs]]></category><category><![CDATA[websocket]]></category><category><![CDATA[socket.io]]></category><category><![CDATA[request tracing]]></category><category><![CDATA[adapter]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Sun, 01 Jan 2023 21:27:28 GMT</pubDate><media:content url="https://nooptoday.com/content/images/2023/03/paws.jpg" medium="image"/><content:encoded><![CDATA[<img src="https://nooptoday.com/content/images/2023/03/paws.jpg" alt="How to Trace Websocket Events in NestJS - [Case Study]"><p>In NestJS, HTTP requests are traced by <code>req.id</code> parameter. You can write your custom id generator, or use default id generated by NestJS or Express or Fastify. However, there is no such concept of request id, or request tracing when it comes to Websockets / Socket.IO.</p><p>Though NestJS allows use of interceptors, filters or guards in <code>Gateways</code>, one feature I&apos;ve felt missing is request id. Without request id, you can&apos;t trace websocket events and that is not very good for observability.</p><p>In this post, I will explain how to trace websocket events in NestJS. For this, we will create a custom <code>WebSocketAdapter</code>. In doing so, we will dive deep into source code of NestJS and solve some challenges!</p><hr><h3 id="what-is-a-websocketadapter">What is a WebSocketAdapter?</h3><p>NestJS makes use of <a href="https://refactoring.guru/design-patterns/adapter?ref=noop-today">Adapter Pattern</a> to abstract underlying libraries from your code. For example NestJS doesn&apos;t care if you are using Fastify or Express as an HTTP library. If you want to use another library, you just need to implement an adapter defined by NestJS.</p><p>There are various websocket libraries in Node.js, and also there is Socket.IO, which makes things easier for developers. In this post, we will modify <code>IoAdapter</code>, from the existing official NestJS package: <a href="https://www.npmjs.com/package/@nestjs/platform-socket.io?ref=noop-today">@nestjs/platform-socket.io</a>.</p><div class="kg-card kg-callout-card kg-callout-card-grey"><div class="kg-callout-emoji">&#x1F4A1;</div><div class="kg-callout-text">Socket.IO is a bidirectional communication protocol, than can work over HTTP or Websockets. We will use it as an example websocket library in this post, but actual details of the Socket.IO protocol is out of scope for this post. I&apos;m planning to publish an article about how Socket.IO works, if you don&apos;t want to miss out, subscribe to <a href="https://nooptoday.com/">Noop Today</a>!</div></div><h2 id="how-to-write-a-websocketadapter-in-nestjs">How to Write a WebSocketAdapter in NestJS</h2><p>To create a custom <code>WebSocketAdapter</code> in NestJS, you need to implement <code>WebSocketAdapter</code> interface from <a href="https://www.npmjs.com/package/@nestjs/common?ref=noop-today">@nestjs/common</a>.</p><figure class="kg-card kg-code-card"><pre><code class="language-typescript">export interface WebSocketAdapter&lt;TServer = any, TClient = any, TOptions = any&gt; {
    create(port: number, options?: TOptions): TServer;
    bindClientConnect(server: TServer, callback: Function): any;
    bindClientDisconnect?(client: TClient, callback: Function): any;
    bindMessageHandlers(client: TClient, handlers: WsMessageHandler[], transform: (data: any) =&gt; Observable&lt;any&gt;): any;
    close(server: TServer): any;
}</code></pre><figcaption><a href="https://github.com/nestjs/nest/blob/master/packages/common/interfaces/websockets/web-socket-adapter.interface.ts?ref=noop-today">https://github.com/nestjs/nest/blob/master/packages/common/interfaces/websockets/web-socket-adapter.interface.ts</a></figcaption></figure><p>In a typical Node.js application involving websockets / socket.io, you probably have the same functionality in a different way. Since we are using NestJS as our framework, lets see how we should structure our code.</p><p>A typical socket.io server in Node.js</p><pre><code class="language-typescript">const express = require(&apos;express&apos;);
const app = express();
const http = require(&apos;http&apos;);
const server = http.createServer(app);
const { Server } = require(&quot;socket.io&quot;);

// create in WebSocketAdapter
const io = new Server(server);

// bindClientConnect in WebSocketAdapter
io.on(&apos;connection&apos;, (socket) =&gt; {
  console.log(&apos;handleConnection&apos;);

  // bindClientDisconnect in WebSocketAdapter
  socket.on(&apos;disconnect&apos;, () =&gt; {
  	console.log(&apos;handleDisconnect&apos;);
  })

  // bindMessageHandlers in WebSocketAdapter
  socket.on(&apos;foo&apos;, (data, callback) =&gt; {
  	console.log(&apos;handleFoo&apos;);
    callback(&apos;OK&apos;);
  })
});

server.listen(3000, () =&gt; {
  console.log(&apos;listening on *:3000&apos;);
});

// close in WebSocketAdapter
setTimeout(() =&gt; { io.close() }, 10000);</code></pre><p>NestJS provides an <code>AbstractWsAdapter</code> class to make things easier for us. So instead of this:</p><pre><code class="language-typescript">import { WebSocketAdapter } from &apos;@nestjs/common&apos;;

export class MyWebSocketAdapter implements WebSocketAdapter {}</code></pre><p>We can do this:</p><pre><code class="language-typescript">import { AbstractWsAdapter } from &apos;@nestjs/websockets&apos;;

export class MyWebsocketAdapter extends AbstractWsAdapter {}</code></pre><p>The difference is that, <code>AbstractWsAdapter</code> provides default implementations for some methods, but you still need to implement <code>create</code> and <code>bindMessageHandlers</code> methods.</p><p>We will act smart and use existing code from the official package, and make modifications only in required parts.</p><h3 id="use-existing-ioadapter-from-nestjsplatform-socketio">Use Existing IoAdapter from @nestjs/platform-socket.io</h3><p>This is the <code>IoAdapter</code> from NestJS official packages, that acts as a bridge between NestJS and Socket.IO.</p><figure class="kg-card kg-code-card"><pre><code class="language-typescript">import { isFunction, isNil } from &apos;@nestjs/common/utils/shared.utils&apos;;
import {
  AbstractWsAdapter,
  MessageMappingProperties,
} from &apos;@nestjs/websockets&apos;;
import { DISCONNECT_EVENT } from &apos;@nestjs/websockets/constants&apos;;
import { fromEvent, Observable } from &apos;rxjs&apos;;
import { filter, first, map, mergeMap, share, takeUntil } from &apos;rxjs/operators&apos;;
import { Server, ServerOptions, Socket } from &apos;socket.io&apos;;

export class IoAdapter extends AbstractWsAdapter {
  public create(
    port: number,
    options?: ServerOptions &amp; { namespace?: string; server?: any },
  ): Server {
    if (!options) {
      return this.createIOServer(port);
    }
    const { namespace, server, ...opt } = options;
    return server &amp;&amp; isFunction(server.of)
      ? server.of(namespace)
      : namespace
      ? this.createIOServer(port, opt).of(namespace)
      : this.createIOServer(port, opt);
  }

  public createIOServer(port: number, options?: any): any {
    if (this.httpServer &amp;&amp; port === 0) {
      return new Server(this.httpServer, options);
    }
    return new Server(port, options);
  }

  public bindMessageHandlers(
    socket: Socket,
    handlers: MessageMappingProperties[],
    transform: (data: any) =&gt; Observable&lt;any&gt;,
  ) {
    const disconnect$ = fromEvent(socket, DISCONNECT_EVENT).pipe(
      share(),
      first(),
    );

    handlers.forEach(({ message, callback }) =&gt; {
      const source$ = fromEvent(socket, message).pipe(
        mergeMap((payload: any) =&gt; {
          const { data, ack } = this.mapPayload(payload);
          return transform(callback(data, ack)).pipe(
            filter((response: any) =&gt; !isNil(response)),
            map((response: any) =&gt; [response, ack]),
          );
        }),
        takeUntil(disconnect$),
      );
      source$.subscribe(([response, ack]) =&gt; {
        if (response.event) {
          return socket.emit(response.event, response.data);
        }
        isFunction(ack) &amp;&amp; ack(response);
      });
    });
  }

  public mapPayload(payload: unknown): { data: any; ack?: Function } {
    if (!Array.isArray(payload)) {
      if (isFunction(payload)) {
        return { data: undefined, ack: payload as Function };
      }
      return { data: payload };
    }
    const lastElement = payload[payload.length - 1];
    const isAck = isFunction(lastElement);
    if (isAck) {
      const size = payload.length - 1;
      return {
        data: size === 1 ? payload[0] : payload.slice(0, size),
        ack: lastElement,
      };
    }
    return { data: payload };
  }
}</code></pre><figcaption><a href="https://github.com/nestjs/nest/blob/master/packages/platform-socket.io/adapters/io-adapter.ts?ref=noop-today">https://github.com/nestjs/nest/blob/master/packages/platform-socket.io/adapters/io-adapter.ts</a></figcaption></figure><p>Let&apos;s see what does all the methods do:</p><ul><li><code>create</code> method is responsible for creating the socket.io server, we don&apos;t need to modify here.</li><li><code>createIoServer</code> is an helper method for <code>create</code></li><li><code>bindMessageHandlers</code> is responsible for delivering socket.io messages to our handlers. We will modify this for sending extra arguments to our handlers.</li><li><code>mapPayload</code> parses payload from socket.io to <code>{ ack, data }</code>, we don&apos;t need to modify here.</li></ul><p>So, we just need to focus on <code>bindMessageHandlers</code>.</p><h3 id="modifying-the-bindmessagehandlers">Modifying The bindMessageHandlers</h3><pre><code class="language-typescript">  public bindMessageHandlers(
    socket: Socket,
    handlers: MessageMappingProperties[],
    transform: (data: any) =&gt; Observable&lt;any&gt;,
  ) {
    const disconnect$ = fromEvent(socket, DISCONNECT_EVENT).pipe(
      share(),
      first(),
    );

    handlers.forEach(({ message, callback }) =&gt; {
      const source$ = fromEvent(socket, message).pipe(
        mergeMap((payload: any) =&gt; {
          const { data, ack } = this.mapPayload(payload);
          return transform(callback(data, ack)).pipe(
            filter((response: any) =&gt; !isNil(response)),
            map((response: any) =&gt; [response, ack]),
          );
        }),
        takeUntil(disconnect$),
      );
      source$.subscribe(([response, ack]) =&gt; {
        if (response.event) {
          return socket.emit(response.event, response.data);
        }
        isFunction(ack) &amp;&amp; ack(response);
      });
    });
  }</code></pre><p>First, lets understand what is <code>handlers</code> in this. It has the type: <code>MessageMappingProperties</code>:</p><pre><code class="language-typescript">export interface MessageMappingProperties {
    message: any;
    methodName: string;
    callback: (...args: any[]) =&gt; Observable&lt;any&gt; | Promise&lt;any&gt; | any;
}</code></pre><p>In socket.io terms: <code>socket.on(message, callback)</code>, <em>message</em> will match the event name, and <em>callback</em> will be the actual handler function.</p><p>So, in order to pass parameters to our handler methods, we must pass parameters to the <code><strong>callback</strong></code>.</p><p>Lets take a closer look at code with additional commentary:</p><pre><code class="language-typescript">// message --&gt; eventName
// callback --&gt; eventHandler
handlers.forEach(({ message, callback }) =&gt; {
	// RxJS equivelant for --&gt; socket.on(message)
    const source$ = fromEvent(socket, message).pipe(
    	// payload is socket.io payload
        mergeMap((payload: any) =&gt; {
            const { data, ack } = this.mapPayload(payload);
            // transform, transforms our callback handler to observable
            // callback function is called with TWO parameters
            return transform(callback(data, ack)).pipe(
                filter((response: any) =&gt; !isNil(response)),
                // If callback returns any response, wrap it with ack
                map((response: any) =&gt; [response, ack]),
            );
        }),
        takeUntil(disconnect$),
    );
    source$.subscribe(([response, ack]) =&gt; {
    	// If you return { event: &apos;foo&apos;, data: &apos;bar&apos; } from handler
        // See: https://docs.nestjs.com/websockets/gateways#multiple-responses
        if (response.event) {
            return socket.emit(response.event, response.data);
        }
        // If client accepts response, send response
        isFunction(ack) &amp;&amp; ack(response);
    });
});</code></pre><h3 id="a-little-problem-in-nestjs">A Little Problem in NestJS</h3><p>If you&apos;ve ever used IoAdapter in your project, you might realize even though the adapter passes two arguments to the callback function, your handler methods don&apos;t receive them in full. Let me explain:</p><p>Say you have a message handler:</p><pre><code class="language-typescript">class MySocketGateway {
	@SubscribeMessage(&apos;foo&apos;)
    fooHandler(argument1, argument2, argument3){
    	console.log({
        	argument1, // Client
            argument2, // Body
            argument3, // undefined
        })
    }
}</code></pre><p>If you go ahead and try something like this, you will see that 1st argument is Socket / Client object, 2nd argument is body BUT 3rd argument is undefined. Two questions come to mind:</p><ul><li>Where did 1st argument came from?</li><li>Where did my ack function disappear?</li></ul><p>Lets go with the first question, if we dive deep into underlying NestJS code, we can find the exact place where <code>adapter.bindMessageHandlers</code> is called.</p><figure class="kg-card kg-code-card"><pre><code class="language-typescript">public subscribeMessages&lt;T = any&gt;(
    subscribersMap: MessageMappingProperties[],
    client: T,
    instance: NestGateway,
  ) {
    const adapter = this.config.getIoAdapter();
    const handlers = subscribersMap.map(({ callback, message }) =&gt; ({
      message,
      callback: callback.bind(instance, client),
    }));
    adapter.bindMessageHandlers(client, handlers, data =&gt;
      fromPromise(this.pickResult(data)).pipe(mergeAll()),
    );
  }</code></pre><figcaption>https://github.com/nestjs/nest/blob/21bd8c37364a2a2591e3de9bfb88d32d09431438/packages/websockets/web-sockets-controller.ts#L147</figcaption></figure><p>If you look closely, callback is bound with two parameters. <code>bind</code> method has one required argument for <code>this</code>, and rest of the arguments are supplied as arguments to the function - <em>callback. </em>More info about this in <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/bind?ref=noop-today">mozilla docs</a>. So, our 1st argument is fixed by NestJS to be client object.</p><h4 id="how-about-ack-function-why-does-it-disapper">How about ack function, why does it disapper?</h4><p>This was very hard for me to debug but I&apos;ve found why does that happen, and how you can fix it!</p><p>The problem is not about the ack function. I&apos;ve tried sending different parameters, or sending more than two parameters to callback function but only the first parameter ends up in handler, and rest is gone.</p><p>There is a helper class <code><a href="https://github.com/nestjs/nest/blob/aa3ad07c1023b71edda6b6ea53374787d3712231/packages/websockets/context/ws-context-creator.ts?ref=noop-today#L56">WsContextCreator</a></code> in <code>@nestjs/websockets</code>, that is responsible for attaching <code>Interceptors</code>, <code>Guards</code>, <code>Pipes</code> and <code>ExceptionHandlers</code> to handlers, <strong>AND managing parameters sent to handler methods!</strong></p><p>That helper class is responsible for disappearing parameters! The reason it makes other parameters disappear is related to this use case:</p><pre><code class="language-typescript">class MyWebsocketGateway {
    @SubscribeMessage(&apos;foo&apos;)
    handleFoo(client: Client, body){
    	console.log({ body });
    }

    @SubscribeMessage(&apos;bar&apos;)
    handleBar(@MessageBody() body){
    	console.log({ body })
    }
}</code></pre><p>If you want to decorate your handler parameters with <code>@MessageBody</code> or <code>@ConnectedSocket</code>, the <code>WsContextCreator</code> assigns those parameters for you. You can even do experiments like this:</p><pre><code class="language-typescript">class MyWebsocketGateway {
	@SubscribeMessage(&apos;foo&apos;)
    handleFoo(arg1, arg2, arg3, @MessageBody() body){
    	console.log({ arg1, arg2, arg3 }) // All undefined
        console.log({ body })
    }
}</code></pre><p>I won&apos;t go into line by line details of how does this happen, but I will provide you a conceptual explanation and if you want to investigate more, you can always look at source code of @nestjs/websockets for a good exercise.</p><p>The key point is <code>WsContextCreator</code> extracts data from initial arguments to the callback function, rearranges them and sends those arguments to your handler function. During the extraction process, <code>WsContextCreator</code> looks up metadata of the handler function:</p><figure class="kg-card kg-code-card"><pre><code class="language-typescript">const metadata = this.contextUtils.reflectCallbackMetadata&lt;TMetadata&gt;(
    instance,
    methodName,
    PARAM_ARGS_METADATA,
) || DEFAULT_CALLBACK_METADATA;</code></pre><figcaption>https://github.com/nestjs/nest/blob/aa3ad07c1023b71edda6b6ea53374787d3712231/packages/websockets/context/ws-context-creator.ts#L163</figcaption></figure><p>If your function doesn&apos;t have <code>PARAM_ARGS_METADATA</code>, NestJS assigns default callback metadata. Guess what, <code>DEFAULT_CALLBACK_METADATA</code> assigns only two parameters to our handler parameters - <em>client and payload.</em></p><figure class="kg-card kg-code-card"><pre><code class="language-typescript">import { WsParamtype } from &apos;../enums/ws-paramtype.enum&apos;;

export const DEFAULT_CALLBACK_METADATA = {
  [`${WsParamtype.PAYLOAD}:1`]: { index: 1, data: undefined, pipes: [] },
  [`${WsParamtype.SOCKET}:0`]: { index: 0, data: undefined, pipes: [] },
};</code></pre><figcaption>https://github.com/nestjs/nest/blob/master/packages/websockets/context/ws-metadata-constants.ts</figcaption></figure><h3 id="solution-for-providing-additional-parameters-to-handlers">Solution for Providing Additional Parameters to Handlers</h3><p>NestJS exports an undocumented helper function called <code>assignCustomParameterMetadata</code>, which allows us to override <code>DEFAULT_CALLBACK_METADATA</code> and use our own metadata.</p><figure class="kg-card kg-code-card"><pre><code class="language-typescript">export function assignCustomParameterMetadata(
  args: Record&lt;number, RouteParamMetadata&gt;,
  paramtype: number | string,
  index: number,
  factory: CustomParamFactory,
  data?: ParamData,
  ...pipes: (Type&lt;PipeTransform&gt; | PipeTransform)[]
) {
  return {
    ...args,
    [`${paramtype}${CUSTOM_ROUTE_AGRS_METADATA}:${index}`]: {
      index,
      factory,
      data,
      pipes,
    },
  };
}</code></pre><figcaption>https://github.com/nestjs/nest/blob/5aeb40b/packages/common/utils/assign-custom-metadata.util.ts#L9</figcaption></figure><p>The function is not self-explanatory, but I think it will be much more clear after example below. Important thing is &quot;where should we use this function?&quot;.</p><p>This function is meant to modify <code>PARAM_ARGS_METADATA</code>, and normally that metadata is defined by decorators such as <code>@ConnectedSocket</code> or <code>@MessageBody</code>. So, lets comply with the conventions and create ourselves a new parameter decorator: <code>@Ack</code>.</p><h3 id="creating-the-ack-decorator">Creating the @Ack Decorator</h3><p>If you are not familiar with the concept of decorators, or decorators in NestJS, you can read my previous article: <a href="https://nooptoday.com/custom-decorators-in-nestjs/">Using Custom Decorators in NestJS</a>.</p><pre><code class="language-typescript">import { PARAM_ARGS_METADATA } from &apos;@nestjs/websockets/constants&apos;;

export function Ack(): ParameterDecorator {
// index is the index of parameter we decorated
// It is not related to getArgByIndex below!
    return function(target, key, index) {
        // Get existing metadata of the handler
        const args = Reflect.getMetadata(PARAM_ARGS_METADATA, target.constructor, key) || {};
        // Extend with new metadata
        const meta = assignCustomParameterMetadata(args, &apos;Ack&apos;, index, (data, input: ExecutionContext) =&gt; {
            // This allows NestJS to extract required parameter from initial arguments supplied to &apos;callback&apos; function
            // Index here needs to match index of the callback parameters
            // 0 --&gt; Client
            // 1 --&gt; Payload from IoAdapter
            // 2 --&gt; Ack from IoAdapter
            // 0 is always client, but rest of the parameters depend on the underlying adapter.
            return input.getArgByIndex&lt;Function&gt;(2);
        });
        Reflect.defineMetadata(PARAM_ARGS_METADATA, meta, target.constructor, key);
    };
}
</code></pre><p>Now we can reach our <code>ack</code> function from the handler as follows:</p><pre><code class="language-typescript">class MyWebSocketGateway {
	@SubscribeMessage(&apos;foo&apos;)
    fooHandler(@MessageBody() body, @Ack() ack){
    	ack(&apos;foo_response&apos;);
    }
}</code></pre><h3 id="lets-move-on-with-requestid">Lets Move On With RequestId</h3><p>That really was a case study, isn&apos;t it? Lets add a request id to our handler parameters in case we want to trace our requests.</p><p>First we need to modify our adapter, if you remember from above, the adapter supplies arguments to the callback function. We can create a request id in our adapter and supply it to the callback.</p><pre><code class="language-typescript">  // You can make your own implementation for this.
  private createRequestId(){
  	return crypto.randomUUID();
  }

  public bindMessageHandlers(
    socket: Socket,
    handlers: MessageMappingProperties[],
    transform: (data: any) =&gt; Observable&lt;any&gt;,
  ) {
    const disconnect$ = fromEvent(socket, DISCONNECT_EVENT).pipe(
      share(),
      first(),
    );

    handlers.forEach(({ message, callback }) =&gt; {
      const source$ = fromEvent(socket, message).pipe(
        mergeMap((payload: any) =&gt; {
          const { data, ack } = this.mapPayload(payload);
          // &lt;-- Modified Section Start
          const requestId = this.createRequestId();
          return transform(callback(data, ack, requestId))
          // Modified Section End --&gt;
          .pipe(
            filter((response: any) =&gt; !isNil(response)),
            map((response: any) =&gt; [response, ack]),
          );
        }),
        takeUntil(disconnect$),
      );
      source$.subscribe(([response, ack]) =&gt; {
        if (response.event) {
          return socket.emit(response.event, response.data);
        }
        isFunction(ack) &amp;&amp; ack(response);
      });
    });
  }</code></pre><p>That is all the modification our adapter needs. Now we can create another decorator: <code>@RequestId</code></p><pre><code class="language-typescript">import { PARAM_ARGS_METADATA } from &apos;@nestjs/websockets/constants&apos;;

export function RequestId(): ParameterDecorator {
    return function(target, key, index) {
        const args = Reflect.getMetadata(PARAM_ARGS_METADATA, target.constructor, key) || {};
        const meta = assignCustomParameterMetadata(args, &apos;RequestId&apos;, index, (data, input: ExecutionContext) =&gt; {
            return input.getArgByIndex&lt;string&gt;(3);
        });
        Reflect.defineMetadata(PARAM_ARGS_METADATA, meta, target.constructor, key);
    };
}
</code></pre><p>That is exactly the same decorator with <code>@Ack</code> with only difference being <code>return input.getArgByIndex(3);</code>. Now our handler function looks like this:</p><pre><code class="language-typescript">class MyWebSocketGateway {
	@SubscribeMessage(&apos;foo&apos;)
    fooHandler(@MessageBody() body, @Ack() ack, @RequestId() requestId){
    	console.log(&apos;Received request with id: &apos;, requestId);
    	ack(&apos;foo_response&apos;);
    }
}</code></pre><h3 id="next-steps">Next Steps</h3><p>If you don&apos;t want to deal with all of these, I&apos;m currently working on an npm package that has better developer ergonomics compared to official @nestjs/platform-socket.io package. I&apos;m aware that decorating every parameter in handler functions can be ugly and tiring to eyes. I believe there are more beautiful solutions to this problem. I&apos;m trying to figure out which solution would result in better developer experience.</p><p>I would like to hear your opinions about this. Whether you suggest a usage example, or you face other problems with official package, please let me know what you think in the comments.</p><p>Also, if you don&apos;t want to miss out on articles about NestJS and programming in general subscribe to <a href="https://nooptoday.com/">Noop Today</a>. I hope you learned something new today, see you in the upcoming posts!</p><hr><p>Here is the repo that contains <code>TraceableIoAdapter</code> with example code: <a href="https://github.com/nooptoday/nestjs-trace-websocket-events?ref=noop-today">https://github.com/nooptoday/nestjs-trace-websocket-events</a></p>]]></content:encoded></item><item><title><![CDATA[Why Websockets are Hard To Scale?]]></title><description><![CDATA[Implementing scalable websocket servers can be tricky. Because websocket servers are stateful applications and they are harder to manage compared to stateless applications]]></description><link>https://nooptoday.com/why-websockets-are-hard-to-scale/</link><guid isPermaLink="false">639dca208ef0e80001338726</guid><category><![CDATA[websocket]]></category><category><![CDATA[load balancing]]></category><category><![CDATA[scalable system]]></category><category><![CDATA[server]]></category><category><![CDATA[backend]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Thu, 22 Dec 2022 05:43:00 GMT</pubDate><media:content url="https://images.unsplash.com/photo-1523961131990-5ea7c61b2107?crop=entropy&amp;cs=tinysrgb&amp;fit=max&amp;fm=jpg&amp;ixid=MnwxMTc3M3wwfDF8c2VhcmNofDMwfHxkaWdpdGFsfGVufDB8fHx8MTY3MTI5NzA4OA&amp;ixlib=rb-4.0.3&amp;q=80&amp;w=2000" medium="image"/><content:encoded><![CDATA[<img src="https://images.unsplash.com/photo-1523961131990-5ea7c61b2107?crop=entropy&amp;cs=tinysrgb&amp;fit=max&amp;fm=jpg&amp;ixid=MnwxMTc3M3wwfDF8c2VhcmNofDMwfHxkaWdpdGFsfGVufDB8fHx8MTY3MTI5NzA4OA&amp;ixlib=rb-4.0.3&amp;q=80&amp;w=2000" alt="Why Websockets are Hard To Scale?"><p>Websockets provide an important feature: bidirectional communication. Which allows servers to <strong>push </strong>events to clients, without the need of a client request.</p><p>The <strong>bidirectional</strong> nature of websockets is both a grace and a curse! Even though it provides a great ton of use case for websockets, it makes implementing a scalable websocket server a lot harder compared to HTTP servers.</p><blockquote>I think websockets are an important part of the web, and they need more recognition among software development world. I am planning to publish more posts about the websockets, if you don&apos;t want to miss out you can subscribe to my mailing list!</blockquote><div class="kg-card kg-button-card kg-align-center"><a href="https://nooptoday.com/#/portal/signup/free" class="kg-btn kg-btn-accent">Subscribe to Get Notified!</a></div><hr><h2 id="what-makes-websockets-unique">What Makes Websockets Unique?</h2><p>Websocket is an application layer protocol, just like HTTP which is another application layer protocol. Both protocols are implemented over TCP connection. But they have different characteristics and they represent two different countries in the communications world - <em>if that makes sense :) </em></p><p>HTTP carries the flag for request-response based communication model, &#xA0;and Websocket carries the flag for bidirectional communication model.</p><blockquote>Side Note: In an attempt to draw a clearer picture of Websocket, you will see a HTTP vs Websocket comparison throughout the post. But that doesn&apos;t mean they are competing protocols, instead they both have their use cases.</blockquote><p><strong>Characteristics of Websocket:</strong></p><ul><li>Bidirectional communication</li><li>Long lived TCP connection</li><li>Stateful protocol</li></ul><p><strong>Characteristics of HTTP:</strong></p><ul><li>Request response based communication</li><li>Short lived TCP connection</li><li>Stateless protocol</li></ul><h3 id="stateful-vs-stateless-protocols">Stateful vs. Stateless Protocols</h3><p>I&apos;m sure you&apos;ve seen some of the posts about creating <em>stateless, and endlessly scalable backend servers.</em> They tell you to use JWT tokens for stateless authentication, and use lambda functions in your stateless application etc.</p><ul><li>What is this <strong>state</strong> they are talking about and why it is so important when it comes to scaling server applications?</li></ul><p><strong>State</strong> is all the information your application has to <em>remember</em> to function correctly. For example, your application should remember logged in users. 99% of the applications do this ( <em>source: trust me ),</em> and it is called session management.</p><ul><li>Okay, then state is a great thing! Why do people hate it, and always try to make <strong>stateless applications?</strong></li></ul><p>You need to store your state in somewhere, and that <em>somewhere</em> is typically the memory of server. But memory of your application server is not reachable from other servers, and the problem begins. </p><p>Imagine a scenario:</p><ul><li><strong>User A</strong> makes a request to <strong>Server 1</strong>. <strong>Server 1 </strong>authenticates <strong>User A</strong>, and saves its <strong>Session A</strong>, to the memory.</li><li><strong>User A</strong> makes second request to <strong>Server 2</strong>. <strong>Server 2</strong> searches saved sessions but it can&apos;t find <strong>Session A</strong>, because it is stored inside <strong>Server 1</strong>.</li></ul><p>In order for your server to become scalable you need to manage the state outside of your application. For example you can save sessions to a Redis instance. This makes application state available to all the servers via Redis, and <strong>Server 2</strong> can read <strong>Session A</strong> from the Redis.</p><hr><p><strong>Stateful Websocket: </strong>Opening a Websocket connection is like a wedding between the client and the server: the connection stays open until one of the parties close it ( <em>or cheat on it, due to network conditions of course</em> ).</p><p><strong>Stateless HTTP: </strong>On the other hand, HTTP is a heartbreaker, it wants to end everything as fast as possible. Once a HTTP connection is opened, client sends a request and as soon as server responds, the connection is closed.</p><p>Okay, I will stop with the jokes now, but remember Websocket connections are <em>typically</em> long lived, whereas HTTP connections are meant to end asap. The moment you introduce Websockets into your application, it becomes <strong>stateful.</strong></p><div class="kg-card kg-toggle-card" data-kg-toggle-state="close"><div class="kg-toggle-heading"><h4 class="kg-toggle-heading-text">In Case You Wonder</h4><button class="kg-toggle-card-icon"><svg id="Regular" xmlns="http://www.w3.org/2000/svg" viewbox="0 0 24 24"><path class="cls-1" d="M23.25,7.311,12.53,18.03a.749.749,0,0,1-1.06,0L.75,7.311"/></svg></button></div><div class="kg-toggle-content"><p>Even though both HTTP and Websocket are built on top of TCP, one can be stateles, while the other one is stateful. For simplicity, I didn&apos;t want to confuse you with details about TCP. But keep in mind, even in HTTP, underlying TCP connection can be long lived. This is out of scope for this post, but you can learn more about it <a href="https://en.wikipedia.org/wiki/HTTP_persistent_connection?ref=noop-today">here</a></p></div></div><h3 id="cant-i-just-use-a-redis-instance-to-store-sockets">Can&apos;t I Just Use a Redis Instance to Store Sockets?</h3><p>In the previous example with sessions, the solution was simple. Use an external service to store sessions, so every other server can read sessions from there ( Redis Instance ).</p><p>Websockets are a different case, because your state is not only the data about socket, inevitably you are storing <strong>connections</strong> in your server. Every websocket connection is bound to a single server, and there is no way for other servers to send data to that connection.</p><p>Now, comes the second problem, you must have a way for other servers to send message to that websocket connection. For that, you need to have a way to send messages between servers. Luckily, that is already a thing called <strong>message broker</strong>. You can even use Redis pub / sub mechanism to send messages between your servers.</p><p>Let&apos;s summarize what have we discussed so far:</p><ul><li>Websocket connections are stateful</li><li>A websocket server automatically becomes a stateful application</li><li>In order for stateful applications to scale, you need to have an external state storage ( example: Redis )</li><li>Websocket connections are bound to a single server</li><li>Servers need to connect to a message broker to send message to websockets in other servers</li></ul><p><em>Is that it? Adding a Redis instance to my stack solves all the scaling problems with Websockets?</em></p><p>Unfortunately, no. Well, there is another issue with scalable websocket architectures: <strong>Load Balancing</strong></p><h3 id="load-balancing-websockets">Load Balancing Websockets</h3><p>Load balancing is a technique to ensure, all of your servers share somewhat equal amount of load. In a plain HTTP server, this can be implemented with simple algorithms like Round Robin. But that is not ideal for a Websocket server.</p><p>Imagine you have an auto scaling server group. That means, as the load increases new instances are deployed and as the load decreases some instances are closed.</p><p>Since HTTP requests are short lived, the load balances somewhat evenly across all instances even though servers are added / removed.</p><p>Websocket connections are long lived ( persistent ), which means new servers will not take the load off from old servers. Because, old servers are still persisting their websocket connections. As an example, say <strong>Server 1</strong>, has 1000 open websocket connections. Ideally, when a new server <strong>Server 2 </strong>is added, you want to move 500 websocket connections from <strong>Server 1</strong>, to <strong>Server 2</strong>. But that is not possible with traditional load balancers.</p><p>You can drop all websocket connections, and expect clients to reconnect. Then you can have 500 / 500 websocket connection distribution on your servers, but that is a bad solution because:</p><ol><li>Servers will be bombarded with reconnection requests, and server load will fluctuate greatly</li><li>If servers are scaled frequently, clients will reconnect frequently and it can have a negative effect on user experience</li><li>It is not an elegant solution - <em>I know you guys care about this!</em></li></ol><p>The most elegant solution for this problem is called: <strong>Consistent Hashing</strong></p><h2 id="load-balancing-algorithm-consistent-hashing">Load Balancing Algorithm: Consistent Hashing</h2><p>There are various load balancing algorithms out there, but consistent hashing is just from another world.</p><figure class="kg-card kg-image-card"><img src="https://nooptoday.com/content/images/2023/03/image-1.jpeg" class="kg-image" alt="Why Websockets are Hard To Scale?" loading="lazy" width="577" height="432"></figure><p>The basic idea behind load balancing with consistent hashing is that:</p><ul><li>Hash the incoming connection with some property, lets say <strong>userId =&gt; hashValue</strong></li><li>You can then use <strong>hashValue</strong> to determine which server this user should connect to</li></ul><p>This assumes that your hash function evenly distributes <strong>userId</strong> to <strong>hashValue</strong>.</p><p>BUT, there is always a but, isn&apos;t there... Now you still have the problem when you add / remove servers. And the solution is to drop connections when new servers are added or removed.</p><p><em>Wait, what! You just said that was a bad idea? How is that a solution, now?</em></p><p>The beauty of this solution is that, with consistent hashing you don&apos;t have to drop <em>all the connections</em>, but you should just drop only some of the connections. Actually, you drop exactly how many connections you need to drop. Let me explain with a scenario:</p><ul><li>Initially, <strong>Server 1</strong> has 1000 connections</li><li><strong>Server 2</strong> is added</li><li>As soon as <strong>Server 2</strong> is added, <strong>Server 1</strong> runs a rebalancing algorithm</li><li>Rebalance algorithm detects which websocket connections are needed to drop, and if our hash function detects roughly 500 connections that need to go to <strong>Server 2</strong></li><li><strong>Server 1</strong>, emits a reconnect message to those 500 clients, and they connect to <strong>Server 2</strong></li></ul><p><a href="https://www.youtube.com/watch?v=UF9Iqmg94tk&amp;ref=noop-today">Here is a great video by ByteByteGo</a> that explains the concept visually.</p><h3 id="a-much-simpler-and-efficient-solution">A Much Simpler And Efficient Solution</h3><p>Discord manages a lot of Websocket connections. How did they solve the problem with load balancing?</p><p>If you investigate the <a href="https://discord.com/developers/docs/topics/gateway?ref=noop-today#get-gateway">developer docs</a> about how to establish a websocket connection, here is how they do it:</p><ul><li>Send a HTTP GET request to <code>/gateway</code> endpoint, receive available Websocket server urls.</li><li>Connect to Websocket server.</li></ul><p>The magic behind this solution is, you can control which server new clients should connect. If you add new server, you can direct all the new connections to new server. If you want to move 500 connections from <strong>Server 1 </strong>to <strong>Server 2</strong>, simply drop 500 connections from <strong>Server 1</strong>, and supply <strong>Server 2</strong> address from <code>/gateway</code> endpoint.</p><p><code>/gateway</code> endpoint needs to know load distributions of all the servers, and make decisions based on that. It can simply return url of the server with minimum load. </p><p>This solution works and much simpler compared to consistent hashing. But, consistent hashing method doesn&apos;t need to know about load distribution of all the servers, and it doesn&apos;t require a HTTP request before hand. Therefore, clients can connect faster but that is generally not an important consideration. Also, implementing a consistent hashing algorithm can be tricky. That is why, I am planning to create a follow up post about Implementing Consistent Hashing for Load Balancing Websockets.</p><p>I hope you learned something new from this post, please let me know what you think in the comments. You can subscribe to mailing list if you don&apos;t want to miss out on new posts!</p>]]></content:encoded></item><item><title><![CDATA[Best Way to Create Dynamic Modules in NestJS]]></title><description><![CDATA[Creating dynamic modules in NestJS can be complex. Use this method and you will never be afraid of creating dynamic modules again!]]></description><link>https://nooptoday.com/dynamic-modules-in-nestjs/</link><guid isPermaLink="false">6398ad3c8ef0e80001338501</guid><category><![CDATA[nestjs]]></category><category><![CDATA[nodejs]]></category><category><![CDATA[typescript]]></category><category><![CDATA[javascript]]></category><category><![CDATA[module]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Tue, 13 Dec 2022 18:34:04 GMT</pubDate><media:content url="https://images.unsplash.com/photo-1590600424262-38d39f98b898?crop=entropy&amp;cs=tinysrgb&amp;fit=max&amp;fm=jpg&amp;ixid=MnwxMTc3M3wwfDF8c2VhcmNofDZ8fHBpbG90fGVufDB8fHx8MTY3MDk1NjI2NQ&amp;ixlib=rb-4.0.3&amp;q=80&amp;w=2000" medium="image"/><content:encoded><![CDATA[<img src="https://images.unsplash.com/photo-1590600424262-38d39f98b898?crop=entropy&amp;cs=tinysrgb&amp;fit=max&amp;fm=jpg&amp;ixid=MnwxMTc3M3wwfDF8c2VhcmNofDZ8fHBpbG90fGVufDB8fHx8MTY3MDk1NjI2NQ&amp;ixlib=rb-4.0.3&amp;q=80&amp;w=2000" alt="Best Way to Create Dynamic Modules in NestJS"><p>NestJS module system is very helpful. They allow us to organize our code, and they help us define module boundaries. Within <code>@Module</code> you can define everything about your module. From the NestJS docs:</p><figure class="kg-card kg-code-card"><pre><code class="language-typescript">import { Module, Global } from &apos;@nestjs/common&apos;;
import { CatsController } from &apos;./cats.controller&apos;;
import { CatsService } from &apos;./cats.service&apos;;
import { CommonModule } from &apos;../common/common.module&apos;;

@Global()
@Module({
  imports: [CommonModule],
  controllers: [CatsController],
  providers: [CatsService],
  exports: [CatsService],
})
export class CatsModule {}</code></pre><figcaption><a href="https://docs.nestjs.com/modules?ref=noop-today#global-modules">https://docs.nestjs.com/modules#global-modules</a></figcaption></figure><p>But things get complicated when you want to configure your module dynamically. This is typically required by database modules but there are many other cases that require you to create dynamic module in NestJS.</p><p>As a convention dynamic modules in NestJS have static methods such as <code>forRoot</code>, <code>forRootAsync</code>, <code>forFeature</code> In this post you will learn how to create a dynamic module in NestJS with all the benefits but with least complexity!</p><hr><h3 id="nestjs-official-docs-are-too-complex">NestJS Official Docs are Too Complex</h3><p>Here is a dynamic module example from the docs:</p><figure class="kg-card kg-code-card"><pre><code class="language-typescript">import { Module } from &apos;@nestjs/common&apos;;
import { ConfigService } from &apos;./config.service&apos;;
import {
  ConfigurableModuleClass,
  ASYNC_OPTIONS_TYPE,
  OPTIONS_TYPE,
} from &apos;./config.module-definition&apos;;

@Module({
  providers: [ConfigService],
  exports: [ConfigService],
})
export class ConfigModule extends ConfigurableModuleClass {
  static register(options: typeof OPTIONS_TYPE): DynamicModule {
    return {
      // your custom logic here
      ...super.register(options),
    };
  }

  static registerAsync(options: typeof ASYNC_OPTIONS_TYPE): DynamicModule {
    return {
      // your custom logic here
      ...super.registerAsync(options),
    };
  }
}</code></pre><figcaption><a href="https://docs.nestjs.com/fundamentals/dynamic-modules?ref=noop-today#extending-auto-generated-methods">https://docs.nestjs.com/fundamentals/dynamic-modules#extending-auto-generated-methods</a></figcaption></figure><p>I know I&apos;ve picked the most bloated example but still, creating a dynamic module as documented in NestJS docs, doesn&apos;t look simple. There is a good reason for that. On one hand NestJS tries to provide a simple API for using module system, but on the other hand the API must provide enough access to satisfy custom needs of developers. I think they did a great job at designing their API, but this is the reason examples from official docs are complex.</p><h3 id="golevelup-team-to-the-rescue">@golevelup Team To The Rescue</h3><p><a href="https://github.com/golevelup/nestjs?ref=noop-today">@golevelup</a> team has some great packages to use along your NestJS project. We will use <code>@golevelup/nestjs-modules</code> for this post. We will use defaults provided by their package, in exchange our dynamic modules will be much simpler and easier to maintain.</p><p>I assume you&apos;ve already spin up a NestJS project. We need to install <code>@golevelup/nestjs-modules</code> package to our project:</p><pre><code class="language-bash">npm install @golevelup/nestjs-modules</code></pre><h3 id="what-will-we-build">What Will We Build</h3><p>For the sake of providing a good use case, lets create a serializer module that can use alternative strategies. Create the following files:</p><pre><code class="language-bash"># Inside src folder
/-&gt; serde
/--&gt; serde.module.ts
/--&gt; serde.service.ts
/--&gt; json.provider.ts
/--&gt; msgpack.provider.ts
/--&gt; constants.ts
/--&gt; interfaces.ts</code></pre><p>This is how we will continue:</p><ul><li>Create a configurable dynamic module <code>SerdeModule</code></li><li>Inject <code>JsonProvider</code> and <code>MsgpackProvider</code> to <code>SerdeService</code></li><li>Define <code>SerdeModuleOptions</code></li><li>Expose selected strategy from <code>SerdeService</code></li></ul><hr><h2 id="creating-configurable-dynamic-module">Creating Configurable Dynamic Module</h2><p>Create your module as usual:</p><pre><code class="language-typescript">// serde.module.ts
@Module({})
export class SerdeModule {}</code></pre><p>Add providers to your module:</p><pre><code class="language-typescript">import { JsonProvider } from &apos;./json.provider&apos;;
import { MsgpackProvider } from &apos;./msgpack.provider&apos;;
import { SerdeService } from &apos;./serde.service&apos;;

@Module({
	providers: [
    	JsonProvider,
        MsgpackProvider,
        SerdeService,
    ]
})
export class SerdeModule {}</code></pre><p>One last thing, we will expose <code>SerdeModule</code>&apos;s functionality from <code>SerdeService</code>. Therefore it needs to be exported:</p><pre><code class="language-typescript">import { JsonProvider } from &apos;./json.provider&apos;;
import { MsgpackProvider } from &apos;./msgpack.provider&apos;;
import { SerdeService } from &apos;./serde.service&apos;;

@Module({
	providers: [
    	JsonProvider,
        MsgpackProvider,
        SerdeService,
    ],
    exports: [SerdeService]
})
export class SerdeModule {}</code></pre><p>So far so good, and there is nothing unusual. Now we will make <code>SerdeModule</code> configurable, specifically we want user to decide which serialization strategy they want to use.</p><p>We need to create an interface to define module options:</p><pre><code class="language-typescript">// interfaces.ts
export interface SerdeModuleOptions {
	strategy: &apos;json&apos; | &apos;msgpack&apos;;
}</code></pre><p>Lastly, we will need these options to be accessible within <code>SerdeService</code>. If you remember from the NestJS docs, module options are injected within providers. And <code>@Inject</code> decorator provides values via an <code>InjectionToken</code>. Injection Token acts like a unique id of the value you want to access. It is defined as follows:</p><pre><code class="language-typescript">declare type InjectionToken = string | symbol | Type;

export interface Type&lt;T = any&gt; extends Function {
    new (...args: any[]): T;
}
</code></pre><p><code>Type</code> is basically a class reference. Since we defined our module options as an interface, we will define a string as Injection Token:</p><pre><code class="language-typescript">// constants.ts
export const SERDE_MODULE_OPTIONS_TOKEN = &apos;SERDE_MODULE_OPTIONS_TOKEN&apos;;</code></pre><p>Finally we can bring them all together:</p><pre><code class="language-typescript">import { JsonProvider } from &apos;./json.provider&apos;;
import { MsgpackProvider } from &apos;./msgpack.provider&apos;;
import { SerdeService } from &apos;./serde.service&apos;;

import { createConfigurableDynamicRootModule } from &apos;@golevelup/nestjs-modules&apos;;
import { SERDE_MODULE_OPTIONS_TOKEN } from &apos;./constants&apos;;
import { SerdeModuleOptions } from &apos;./interfaces&apos;;

@Module({
	providers: [
    	JsonProvider,
        MsgpackProvider,
        SerdeService,
    ],
    exports: [SerdeService]
})
export class SerdeModule extends createConfigurableDynamicRootModule&lt;SerdeModule, SerdeModuleOptions&gt;(SERDE_MODULE_OPTIONS_TOKEN) {}</code></pre><p>That is all! We&apos;ve successfully created a dynamic module. Lets put it into use, we can import it within <code>AppModule</code>:</p><pre><code class="language-typescript">// app.module.ts
import { SerdeModule } from &apos;./serde.module&apos;;

@Module({
	imports: [
		SerdeModule.forRoot(SerdeModule, { strategy: &apos;json&apos; })
    ]
})
</code></pre><p>Notice we&apos;ve never defined a <code>forRoot</code> method on our class. This method is automatically created by <code>@golevelup/nestjs-modules</code> and it is type safe, Hurray! There is also the <code>forRootAsync</code> counterpart:</p><pre><code class="language-typescript">// app.module.ts
import { SerdeModule } from &apos;./serde.module&apos;;
import { ConfigModule, ConfigService } from &apos;@nestjs/config&apos;;


@Module({
	imports: [
		SerdeModule.forRootAsync(SerdeModule, {
        	imports: [ConfigModule],
            inject: [ConfigService],
            useFactory: (configService: ConfigService) =&gt; {
            	return {
                	strategy: configService.get(&apos;SERDE_STRATEGY&apos;)
                }
            }
        })
    ]
})
</code></pre><h2 id="accessing-module-options">Accessing Module Options</h2><p>To make use of provided options, we need to inject it into <code>SerdeService</code></p><pre><code class="language-typescript">import { SERDE_MODULE_OPTIONS_TOKEN } from &apos;./constants&apos;;
import { SerdeModuleOptions } from &apos;./interfaces&apos;;

@Injectable()
export class SerdeService {
	constructor(
    	@Inject(SERDE_MODULE_OPTIONS_TOKEN)
        moduleOptions: SerdeModuleOptions
    ){
    	console.log({ moduleOptions });
    }
}

// console output: { moduleOptions: { strategy: &apos;json&apos; } }</code></pre><p>We will apply strategy pattern for this example. If you want to learn more about strategy pattern, I recommend you to read <a href="https://betterprogramming.pub/design-patterns-using-the-strategy-pattern-in-javascript-3c12af58fd8a?ref=noop-today">https://betterprogramming.pub/design-patterns-using-the-strategy-pattern-in-javascript-3c12af58fd8a</a> written by Carlos Caballero. Basically we will switch between <code>JsonProvider</code> and <code>MsgpackProvider</code> depending on the <code>moduleOptions.strategy</code>.</p><p>First inject the providers into <code>SerdeService</code></p><pre><code class="language-typescript">import { SERDE_MODULE_OPTIONS_TOKEN } from &apos;./constants&apos;;
import { SerdeModuleOptions } from &apos;./interfaces&apos;;

import { JsonProvider }&#xA0;from &apos;./json.provider&apos;;
import { MsgpackProvider }&#xA0;from &apos;./msgpack.provider&apos;;

@Injectable()
export class SerdeService {
	private readonly _strategy;

	constructor(
    	@Inject(SERDE_MODULE_OPTIONS_TOKEN)
        moduleOptions: SerdeModuleOptions,
        private readonly jsonProvider: JsonProvider,
        private readonly msgpackProvider: MsgpackProvider,
    ){
    	switch(moduleOptions.strategy) {
        	case &apos;json&apos;:
            	this._strategy = jsonProvider;
                break;
            case &apos;msgpack&apos;:
            	this._strategy = msgpackProvider;
                break;
        }
    }
}</code></pre><p>Now, we can expose our API to outside world:</p><pre><code class="language-typescript">import { SERDE_MODULE_OPTIONS_TOKEN } from &apos;./constants&apos;;
import { SerdeModuleOptions } from &apos;./interfaces&apos;;

import { JsonProvider }&#xA0;from &apos;./json.provider&apos;;
import { MsgpackProvider }&#xA0;from &apos;./msgpack.provider&apos;;

@Injectable()
export class SerdeService {
	private readonly _strategy;

	constructor(
    	@Inject(SERDE_MODULE_OPTIONS_TOKEN)
        moduleOptions: SerdeModuleOptions,
        private readonly jsonProvider: JsonProvider,
        private readonly msgpackProvider: MsgpackProvider,
    ){
    	switch(moduleOptions.strategy) {
        	case &apos;json&apos;:
            	this._strategy = jsonProvider;
                break;
            case &apos;msgpack&apos;:
            	this._strategy = msgpackProvider;
                break;
        }
    }

 	public serialize(data) {
    	return this._strategy.serialize(data);
    }

    public parse(data) {
    	return this._strategy.parse(data);
    }
}</code></pre><blockquote>I leave implementing actual providers to you. If you want to learn more about Strategy Pattern, I am planning to write a detailed post about it. Tell me in the comments if you are interested!</blockquote><hr><p>We created a dynamic module in NestJS with an example use case. I hope you&apos;ve learned something new, let me know what you think in the comments!</p>]]></content:encoded></item><item><title><![CDATA[Using Custom Decorators in NestJS]]></title><description><![CDATA[Use full potential of decorators in your NestJS projects. In this post, you will learn how to create custom decorators in NestJS]]></description><link>https://nooptoday.com/custom-decorators-in-nestjs/</link><guid isPermaLink="false">6387e498d156c600012ff492</guid><category><![CDATA[nestjs]]></category><category><![CDATA[typescript]]></category><category><![CDATA[javascript]]></category><category><![CDATA[nodejs]]></category><category><![CDATA[decorator]]></category><dc:creator><![CDATA[Muhammed Kadir Tan]]></dc:creator><pubDate>Sat, 03 Dec 2022 10:58:31 GMT</pubDate><media:content url="https://images.unsplash.com/photo-1550439062-609e1531270e?crop=entropy&amp;cs=tinysrgb&amp;fit=max&amp;fm=jpg&amp;ixid=MnwxMTc3M3wwfDF8c2VhcmNofDV8fHByb2dyYW1taW5nfGVufDB8fHx8MTY2OTg1MDQxMQ&amp;ixlib=rb-4.0.3&amp;q=80&amp;w=2000" medium="image"/><content:encoded><![CDATA[<img src="https://images.unsplash.com/photo-1550439062-609e1531270e?crop=entropy&amp;cs=tinysrgb&amp;fit=max&amp;fm=jpg&amp;ixid=MnwxMTc3M3wwfDF8c2VhcmNofDV8fHByb2dyYW1taW5nfGVufDB8fHx8MTY2OTg1MDQxMQ&amp;ixlib=rb-4.0.3&amp;q=80&amp;w=2000" alt="Using Custom Decorators in NestJS"><p><a href="https://nestjs.com/?ref=noop-today">NestJS</a> is a great framework for developing Node.js server applications. Your code looks clean and descriptive thanks to <a href="https://angular.io/guide/dependency-injection?ref=noop-today">Dependency Injection</a> ( DI ) system and extensive use of decorators in NestJS. Here is a sample code from a NestJS controller:</p><pre><code class="language-typescript">// app.controller.ts

@Controller(&apos;app&apos;)
export class AppController {
	constructor(private readonly appService: AppService){}

    @Post(&apos;auth-user&apos;)
    authUser(@Body() body: AuthDto){
    	return this.appService.authUser(body.email, body.password)
    }
}</code></pre><p>The code just explains itself, we have a router for the path &quot;app&quot; and it accepts a post request at path &quot;auth-user&quot;. Request body is parsed into the &quot;body&quot; variable and it passes email and password down to the UserService.</p><h2 id="why-you-should-use-decorators">Why You Should Use Decorators?</h2><h4 id="point-1-loose-coupling">Point 1: Loose Coupling</h4><p>The example code above, doesn&apos;t involve any http server library such as <a href="https://expressjs.com/?ref=noop-today">express</a> or <a href="https://www.fastify.io/?ref=noop-today">fastify</a>. However, your NestJS application most likely use one of them! </p><p>When one part of the application <em>( e.g. controller )</em> doesn&apos;t need to know about other part of your application <em>( e.g. express server )</em> it is said your code is <a href="https://www.brainspire.com/blog/what-is-loose-coupling-why-does-it-matter-for-your-new-project?ref=noop-today">loosely coupled</a>. Loose coupling has advantages / disadvantages over tight coupling, but generally it is preferred where your software needs flexibility. </p><p>For example in NestJS you can switch underlying http server without changing single line of your controllers, because they are independent from each other.</p><h4 id="point-2-cleaner-code">Point 2: Cleaner Code</h4><p>Decorators are simple one-liners that are easy to understand and read. However, if you rely on decorators too much, your methods will sit under numerous decorators. As they say with everything, don&apos;t overuse it but if you use it when really needed, they will make your code cleaner.</p><p>As a rule of thumb, use decorators for stable functionalities in your app. Don&apos;t adopt them early in the development process, but once you feel like you are repeating same code again and again, it is a possible use case for decorators.</p><h4 id="point-3-reduced-overall-complexity">Point 3: Reduced Overall Complexity</h4><p>Surely, decorators are not very simple to implement. You might feel, you have made things even more complex than before. But this is not very accurate, because decorators hide this complexity from your application code, and you only need to implement them once!</p><p>Important thing is, you want to move complexity from your application code to another module or a library. This will allow you to focus on the important stuff.</p><blockquote>Use Decorators for Stable Parts of Your Application<br>Once you create a decorator, you don&apos;t want to add / remove parameters. Because creating a decorator means you are defining a contract / signature and breaking contracts is a bad practice.<br><em>For example, <strong>POST</strong> decorator from NestJS accepts single parameter for the path, and this contract is very stable.</em></blockquote><hr><h2 id="what-will-we-build-today">What will we build today?</h2><p>In this post, we will create a decorator called HandleEvent. It will allow us to bind event handlers to events. It will be very similar to Post decorator from NestJS if you think about it. Responsibility of Post decorator is to bind certain requests to handlers.</p><p>To achieve this we will:</p><ol><li>Create a decorator called: HandleEvent ( actually a decorator factory )</li><li>Create EventHandlerDiscovery, which will discover all the decorated methods within project.</li><li>Create an EventManager, which will be responsible for storing event handlers and firing events.</li><li>Create an EventHandlerService, which will contain the event handler for example event: &apos;new-subscription&apos;</li></ol><blockquote>This example assumes you are already familiar with NestJS, and you need a NestJS project for example codes to work.</blockquote><h2 id="lets-write-a-custom-decorator">Lets write a custom decorator!</h2><p>Decorators are regular functions</p><pre><code class="language-typescript">function MyFirstDecorator(constructor){
	console.log(&apos;I am decorated: &apos;, constructor.name)
}

@MyFirstDecorator
class ExampleClass {
}</code></pre><blockquote>There are various kinds of decorators and you can find all of them here: <a href="https://www.typescriptlang.org/docs/handbook/decorators.html?ref=noop-today">https://www.typescriptlang.org/docs/handbook/decorators.html</a> <br>To keep things simple we will use <strong>method decorators</strong> in this post, but if you would like to see more just leave a comment below.</blockquote><p>Main takeaway is that, different kinds of decorators accept and return different parameters.</p><p>Lets create a <a href="https://www.typescriptlang.org/docs/handbook/decorators.html?ref=noop-today#method-decorators">MethodDecorator</a>.</p><pre><code class="language-typescript">// handle-event.decorator.ts

export function HandleEvent(event: string){
    return function (target, propertyKey, descriptor){
        Reflect.defineMetadata(&apos;HANDLE_EVENT&apos;, event, descriptor.value);
    }
}
</code></pre><p>We will use this decorator to create a map between event handlers and event names. It will be very similar to NestJS route mapping via <strong>@Post(&apos;auth-user&apos;)</strong> decorator.</p><pre><code class="language-typescript">class EventHandlerService {

	@HandleEvent(&apos;new-subscription&apos;)
	newSubscriptionHandler(newSubscriptionEvent){
    	const { email } = newSubscriptionEvent;
    	console.log(`[new-subscription]: ${email}`)
        this.discordHelper.notify(`You have a new subscriber: ${email}`)
    }

}</code></pre><p>So, what is going on here?</p><p>As you can notice, this time we called the decorator function <strong>HandleEvent </strong>with &apos;new-subscription&apos; parameter. We didn&apos;t call <strong>MyFirstDecorator</strong> from earlier example. The truth is, HandleEvent is not a decorator but a decorator factory. It is a function that returns a decorator, <em>i.e. Higher Order Function.</em></p><p>This is a very common pattern in decorators. Because your decorator will most likely require options / parameters to be useful. For HandleEvent decorator to function properly, it needs to know which event are we talking about. So we are accepting the event name as parameter.</p><p><em>Okay, but what does target, propertyKey or descriptor mean?</em></p><p><em>target </em>is a reference to method&apos;s class. In our example it is <strong>EventHandlerService</strong>. propertyKey is the method&apos;s name, in our example it is <strong>newSubscriptionHandler</strong>, and descriptor is a <strong>PropertyDescriptor</strong> and to obtain method reference we need to access <strong>descriptor.value</strong>.</p><p>Code below adds metadata to the method, using Reflect object.</p><pre><code class="language-typescript">Reflect.defineMetadata(&apos;HANDLE_EVENT&apos;, event, descriptor.value);</code></pre><p>Reflect is exported from &#xA0;<strong>reflect-metadata </strong>package. So lets install it. <em>Though you may not need it if your target is above ES2015</em></p><pre><code class="language-bash">npm install reflect-metadata</code></pre><p>There we go, we created our first decorator and as the name suggests, we will use this decorator to map between events and handlers.</p><h3 id="additional-read-how-does-nestjs-handle-decorators-behind-the-scenes">Additional Read: How does NestJS handle decorators behind the scenes?</h3><p>Decorators work by adding metadata to your classes, methods etc.</p><p>At runtime your program can read all the metadata added to your code and do queries like: &quot;Get me all <strong>methods </strong>with a <strong>POST</strong> metadata&quot;</p><p>Result of these queries contain references to actual methods and classed, and that is all we need to make magic happen! </p><p>NestJS handles this in two parts:</p><ul><li><a href="https://github.com/nestjs/nest/blob/master/packages/core/scanner.ts?ref=noop-today">Scanning</a> : NestJS detects all the metadata in your code during scanning.</li><li><a href="https://github.com/nestjs/nest/blob/master/packages/core/discovery/discovery-service.ts?ref=noop-today">Discovery</a><strong> </strong>or <a href="https://github.com/nestjs/nest/blob/master/packages/core/router/router-explorer.ts?ref=noop-today">Exploring</a>: NestJS inspects all metadata, and applies its own decorator logics ( <em>routing, injecting, using pipes etc.</em> ). </li></ul><p>Luckily, we don&apos;t have to write these functionalities from scratch, but it is always good to know what is actually going on behind the scenes.</p><h2 id="detect-your-decorated-methods-with-discovery-service">Detect your decorated methods with Discovery Service</h2><p><a href="https://github.com/golevelup/nestjs?ref=noop-today">@golevelup</a> team has very convenient packages to use alongside your NestJS project. For this example we will use discovery package, lets install it.</p><pre><code>npm install @golevelup/nestjs-discovery</code></pre><p>Now we need to discover our methods as follows:</p><pre><code class="language-typescript">// event-handler.discovery.ts

import { DiscoveryService }&#xA0;from &apos;@golevelup/nestjs-discovery&apos;;

@Injectable()
export class EventHandlerDiscovery {
	// Inject discovery service from @golevelup
	constructor(private readonly discoveryService: DiscoveryService)

    public async getEventsToHandlersMap(){
    	// This returns all the methods decorated with our decorator
    	const scanResult = await this.discoveryService.
        providerMethodsWithMetaAtKey(&apos;HANDLE_EVENT&apos;)

        const eventsToHandlersMap = new Map();

        scanResult.forEach(result =&gt; {
        	const event = result.meta[&apos;HANDLE_EVENT&apos;];
            const handler = result.discoveredMethod.handler;
            const that = result.discoveredMethod.parentClass.instance;
            const boundHandler = handler.bind(that);

            eventsToHandlersMap.set(event, boundHandler);
        })

        return eventsToHandlersMap;
    }
}</code></pre><p>Notice we bind the handler to its parent class. This ensures, any reference to <strong>this</strong> in the handler method points to its own class. Otherwise your handler may not work properly.</p><pre><code>        const boundHandler = handler.bind(that);
</code></pre><p><strong>DiscoveryService</strong> can only check classes inside NestJS Dependency Injection system. In other words, we need to add <strong>EventHandlerService</strong> to the providers array of <strong>AppModule </strong>or any other module. That is simple:</p><pre><code class="language-typescript">// app.module.ts

@Module({
    controllers: [AppController],
    providers: [
        AppService,
        EventHandlerDiscovery,
        EventHandlerService,
    ],
})
export class AppModule {}</code></pre><h2 id="wrapping-all-together">Wrapping All Together</h2><p>Now our decorator and discovery service is setup, there is only one step left. We need to use that eventsToHandlersMap and actually call handler functions from somewhere.</p><pre><code class="language-typescript">@Injectable()
export class EventManager implements OnModuleInit {
	private eventsToHandlersMap;

	constructor(
    	private readonly eventHandlerDiscovery: EventHandlerDisovery
    ){}

    async onModuleInit(){
    	this.eventsToHandlersMap = await this.eventHandlerDiscovery.getEventsToHandlersMap()
    }

    public fireEvent(event, data){
    	const handler = this.eventsToHandlersMap.get(event);
        if(handler){
        	handler(data);
        }
    }

}</code></pre><p>Discovery service works asynchronously, so it is not possible to use it inside constructor. However, we can use <strong>OnModuleInit</strong> interface from NestJS, which allows us to setup our module ( even run asynchronous code inside it ) before it is marked as ready. This way we can obtain eventsToHandlersMap in our class.</p><p>Now any part of our application can use EventManager to fire an event, and they don&apos;t need to import EventHandlerService or any other service that use our decorator.</p><p>Lets actually fire an event from AppService</p><pre><code class="language-typescript">// app.service.ts

@Injectable()
export class AppService {
	subscribers: { email: string }[] = []

	constructor(
    	private readonly eventManager: EventManager,
    ){}

    subscribe(subscribeDto){
    	const newSubscriber = { email: subscribeDto.email }
    	this.subscribers.push(newSubscriber);
        this.eventManager.fireEvent(&apos;new-subscription&apos;, newSubscriber)
    }
}</code></pre><p>There we go, you implemented a Custom Decorator in NestJS. There are many use cases where decorators fit great and reduce complexity, and reduce code coupling. But it should be used with caution, because if you add decorators for everything, your code may end up even more complex!</p><p><strong>AppService</strong> who is going to handle the event. Also <strong>EventHandlerService</strong> has no idea who fired the event. They are now decoupled from each other. Only link between them is EventManager and HandleEvent decorator.</p><p>You can always handle your event from another service, such as <strong>SubscriptionEventsService, </strong> and you can always fire &apos;new-subscription&apos; event from another service. For example you might add users from another endpoint or another data source.</p><hr><p>In the upcoming posts, I will add more detailed use for custom decorators. So, don&apos;t forget to subscribe, have a nice one!</p><p><em>Let me know what you think in the comments, thanks for reading.</em></p>]]></content:encoded></item></channel></rss>
    "#;

  let result = parse(
    String::from(rss),
    Some(String::from("https://nooptoday.com")),
  );
  println!("{:?}", result)
}
