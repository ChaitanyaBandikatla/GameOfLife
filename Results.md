<h1>Results</h1>

<h2>Hypothesis</h2>
<p>A candidate exposed to a familiar programming language or a programming language with similar syntax that they are familiar with, will help them understand and debug code faster.</p>
<h2>Methods</h2>
<ol>
  <li>Initially, we introduced bugs in all the three languages. We focused on one compilation error and two logical errors.</li>
  <li>A Google form for signing up for software evaluation trials with preferred time slots was put up for candidates to sign up.</li>
  <li>The form had information about their expertise and familiarity in the three coding languages Ruby, Rust and Go.</li>
  <li>Using the preferred time slots, we scheduled video calls via zoom for 30 mins where the candidates were expected to try debugging in all the three languages within the timeframe.</li>
  <li>Based on the debugging procedure of the candidates, we recorded observations to note various metrics that would help us test our hypothesis.</li>
</ol>

<h2>Materials</h2>
<ol>
  <li><a href="https://forms.gle/dGQZxhJNkcengY2d9">Sign Up Link Sheet</a></li>
  <li><a href="https://tinyurl.com/y5we29rn">Evaluation Sheet</a></li>
</ol>
<h2>Observations</h2>
<p><b>The following observations were made for Compilation Errors:</b></p>
<ol>
  <li>Time taken to debug compilation errors in Ruby, Rust and Go<br><img src="https://github.com/ChaitanyaBandikatla/GameOfLife/blob/master/img/compilation_debug_time.PNG"><br>Candidates took more time to debug compilation erros in Ruby when compared to Rust and Go</li>
  <li>Time taken by candidates familiar in the language to debug compilation errors<br><img src="https://github.com/ChaitanyaBandikatla/GameOfLife/blob/master/img/compilation_familiar_time.PNG"></li>
  <li>Time taken by candidates unfamiliar in the language to debug compilation errors<img src="https://github.com/ChaitanyaBandikatla/GameOfLife/blob/master/img/compilation_unfamiliar_time.PNG"></li>
</ol>
<p>In our pre-debugging sessions census, we asked the level of familiarity of the 3 languages by each candidate. Based on the results, we found that there  is very minute difference to debug a complication error with prior knowledge of the language.</p>
<p><b>The following observations were made for Logical Errors:</b></p>
<ol>
<li>Time taken to debug logical errors in Ruby, Rust and Go<br><img src="https://github.com/ChaitanyaBandikatla/GameOfLife/blob/master/img/logical_debug_time.PNG"><br>Candiates took the most time to debug logical errors in Rust, followed by Go and then Ruby.</li>
  <li>Time taken by candidates familiar in the language to debug logical errors<br><img src="https://github.com/ChaitanyaBandikatla/GameOfLife/blob/master/img/logical_familiar_time.PNG"></li>
  <li>Time taken by candidates unfamiliar in the language to debug logical errors<br><img src="https://github.com/ChaitanyaBandikatla/GameOfLife/blob/master/img/logical_unfamiliar_time.PNG"></li>
  </ol>
<p>Based on the results, we found that only people who are unfamilair in Rust and Ruby took more time than people knew the language before. The reults for Go seems almost similar with people who know the language or not.</p>
<h2>Conclusion</h2>
<ol>
  <li>Candidates were most familiar with Ruby and least familiar with Rust.</li>
  <li>Candidates found it harder to debug in Rust when compared to Ruby or Go. They found this language to be very unfamiliar and hard to understand. </li>
  <li>Candidates found it easy to debug logical errors in Ruby. Some of them mentioned the syntax to be like Python, hence making the code easily understandable.</li>
  <li>4 out of 9 Candidates used print statements to debug code. 2 candidates referred online resources for debugging.</li>
  <li>Hence, familiarity with the language helped candidates to debug the code faster. The hypothesis we tested against is correct.</li>
</ol>
<h2>Threats to Validity</h2>
<table style="width:100%">
  <tr>
    <th>Threat</th>
    <th>Possible Solution</th>
  </tr>
  <tr>
    <td>We didn't have a benchmark to compare our results with.</td>
    <td>We should have timed and taken the debugging sessions ourselves to observe how we debug the bugs we created.</td>
  </tr>
  <tr>
    <td>We didn't have a standardised code testings as we introduced different kinds of bugs in different languages.</td>
    <td>We should have introduced the same bugs in all the languages and asked the candidates to debug in the language that they are the most familiar with.</td>
  </tr>
  <tr>
    <td>We didn't take a post survey after the debugging sessions.</td>
    <td>Metrics collected from the post survey could have helped us understand more about the candidates perspective while they debugged the code in different languages.</td>
  </tr>
</table>
