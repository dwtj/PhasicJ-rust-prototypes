package phasicj.tracing.truffle_instrument;

import org.graalvm.options.OptionCategory;
import org.graalvm.options.OptionKey;
import org.graalvm.options.OptionStability;
import org.graalvm.options.OptionValues;

import com.oracle.truffle.api.Option;
import com.oracle.truffle.api.instrumentation.TruffleInstrument;
import com.oracle.truffle.api.instrumentation.TruffleInstrument.Registration;

@Registration(id = TracingTruffleInstrument.ID, name = "Tracing Truffle Instrument", version = "0.1", services = TracingTruffleInstrument.class)
public final class TracingTruffleInstrument extends TruffleInstrument {

    public static final String ID = "tracing-truffle-instrument";

    /**
     * Look at {@link #onCreate(Env)} and {@link #getOptionDescriptors()} for more info.
     */
    @Option(name = "enable", help = "Enable instrument (default: false).", category = OptionCategory.USER, stability = OptionStability.STABLE)
    static final OptionKey<Boolean> ENABLED = new OptionKey<>(false);

    @Override
    protected void onCreate(final Env env) {
        final OptionValues options = env.getOptions();
        if (ENABLED.getValue(options)) {
            enable(env);
            env.registerService(this);
        }
    }

    private void enable(final Env env) {
        // TODO(dwtj): Read this copy-pasta.
        //SourceSectionFilter filter = SourceSectionFilter.newBuilder().tagIs(ExpressionTag.class).includeInternal(false).build();
        //Instrumenter instrumenter = env.getInstrumenter();
        //instrumenter.attachLoadSourceSectionListener(filter, new GatherSourceSectionsListener(this), true);
        //instrumenter.attachExecutionEventFactory(filter, new CoverageEventFactory(this));
    }
}
